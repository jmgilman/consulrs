use std::str::FromStr;

use crate::api::features::FeaturedEndpoint;
use crate::client::Client;
use crate::error::ClientError;
use derive_builder::Builder;
use rustify::endpoint::{Endpoint, EndpointResult, MiddleWare};
use rustify::errors::ClientError as RestClientError;
use serde::de::DeserializeOwned;

pub use crate::api::features::Features;

pub mod catalog;
pub mod check;
pub mod configuration;
pub mod connect;
pub mod features;
pub mod health;
pub mod kv;
pub mod service;
pub mod session;
pub mod snapshot;

#[derive(Builder, Debug)]
#[builder(pattern = "owned")]
pub struct ApiResponse<T> {
    #[builder(setter(into, strip_option), default)]
    pub cache: Option<String>,
    #[builder(setter(into, strip_option), default)]
    pub content_hash: Option<String>,
    #[builder(setter(into, strip_option), default)]
    pub default_acl_policy: Option<String>,
    #[builder(setter(into, strip_option), default)]
    pub index: Option<String>,
    #[builder(setter(into, strip_option), default)]
    pub known_leader: Option<String>,
    #[builder(setter(into, strip_option), default)]
    pub last_contact: Option<String>,
    #[builder(setter(into, strip_option), default)]
    pub query_backend: Option<String>,
    pub response: T,
}

impl<T> ApiResponse<T> {
    pub fn builder() -> ApiResponseBuilder<T> {
        ApiResponseBuilder::default()
    }
}

/// A [MiddleWare] for adding version and token information to all requests.
///
/// Implements [MiddleWare] to provide support for prepending API version
/// information to all requests and adding an ACL token to the header of all
/// requests. Additionally, any API features specified in the endpoint are
/// appended to the request. This is passed by the API functions when an
/// endpoint is executed.
#[derive(Debug, Clone)]
pub struct EndpointMiddleware {
    pub features: Option<Features>,
    pub token: Option<String>,
    pub version: String,
}
impl MiddleWare for EndpointMiddleware {
    #[instrument(skip(self, req), err)]
    fn request<E: Endpoint>(
        &self,
        _: &E,
        req: &mut http::Request<Vec<u8>>,
    ) -> Result<(), rustify::errors::ClientError> {
        // Prepend API version to all requests
        debug!(
            "Middleware: prepending {} version to URL",
            self.version.as_str()
        );
        let url = url::Url::parse(req.uri().to_string().as_str()).unwrap();
        let mut url_c = url.clone();
        let mut segs: Vec<&str> = url.path_segments().unwrap().collect();
        segs.insert(0, self.version.as_str());
        url_c.path_segments_mut().unwrap().clear().extend(segs);
        *req.uri_mut() = http::Uri::from_str(url_c.as_str()).unwrap();
        debug!("Middleware: final URL is {}", url_c.as_str());

        // Add ACL token to header if present
        if let Some(token) = &self.token {
            debug!("Middleware: adding ACL token to header");
            req.headers_mut().append(
                "X-Consul-Token",
                http::HeaderValue::from_str(token).unwrap(),
            );
        }

        // Add optional API features
        if let Some(f) = &self.features {
            f.process(req);
        }

        Ok(())
    }

    fn response<E: Endpoint>(
        &self,
        _: &E,
        _: &mut http::Response<Vec<u8>>,
    ) -> Result<(), rustify::errors::ClientError> {
        Ok(())
    }
}

/// Executes an [Endpoint] and returns the raw response body.
///
/// Any errors which occur in execution are wrapped in a
/// [ClientError::RestClientError] and propagated.
pub async fn exec_with_empty<E>(
    client: &impl Client,
    endpoint: E,
) -> Result<ApiResponse<()>, ClientError>
where
    E: Endpoint<Response = ()> + FeaturedEndpoint,
{
    info!("Executing {} and expecting no response", endpoint.path());
    let features = endpoint.features();
    endpoint
        .with_middleware(&client.middle(features))
        .exec(client.http())
        .await
        .map_err(parse_err)
        .map(parse_empty)?
}

/// Executes an [Endpoint] and returns the raw response body.
///
/// Any errors which occur in execution are wrapped in a
/// [ClientError::RestClientError] and propagated.
pub async fn exec_with_raw<E>(
    client: &impl Client,
    endpoint: E,
) -> Result<ApiResponse<Vec<u8>>, ClientError>
where
    E: Endpoint + FeaturedEndpoint,
{
    info!("Executing {} and expecting a response", endpoint.path());
    let features = endpoint.features();
    endpoint
        .with_middleware(&client.middle(features))
        .exec(client.http())
        .await
        .map_err(parse_err)
        .map(parse_raw)?
}

/// Executes an [Endpoint] and returns the result.
///
/// The result from the executed endpoint has a few operations performed on it:
///
/// * Any potential API error responses from the execution are searched for and,
///   if found, converted to a [ClientError::APIError]
/// * All other errors are mapped from [rustify::errors::ClientError] to
///   [ClientError::RestClientError]
pub async fn exec_with_result<E>(
    client: &impl Client,
    endpoint: E,
) -> Result<ApiResponse<E::Response>, ClientError>
where
    E: Endpoint + FeaturedEndpoint,
{
    info!("Executing {} and expecting a response", endpoint.path());
    let features = endpoint.features();
    endpoint
        .with_middleware(&client.middle(features))
        .exec(client.http())
        .await
        .map_err(parse_err)
        .map(parse)?
}

/// Parses an [EndpointResult], turning it into an [ApiResponse].
fn parse<T>(result: EndpointResult<T>) -> Result<ApiResponse<T>, ClientError>
where
    T: DeserializeOwned + Send + Sync,
{
    let mut builder = parse_headers(result.response.headers());

    let response = result.parse().map_err(ClientError::from)?;
    builder = builder.response(response);
    Ok(builder.build().unwrap())
}

/// Parses an [EndpointResult], turning it into an [ApiResponse].
fn parse_empty(result: EndpointResult<()>) -> Result<ApiResponse<()>, ClientError> {
    let mut builder = parse_headers(result.response.headers());

    builder = builder.response(());
    Ok(builder.build().unwrap())
}

/// Parses an [EndpointResult], turning it into an [ApiResponse].
fn parse_raw<T>(result: EndpointResult<T>) -> Result<ApiResponse<Vec<u8>>, ClientError>
where
    T: DeserializeOwned + Send + Sync,
{
    let mut builder = parse_headers(result.response.headers());

    let response = result.raw();
    builder = builder.response(response);
    Ok(builder.build().unwrap())
}

/// Parses commonly found header fields out of response headers.
fn parse_headers<T>(headers: &reqwest::header::HeaderMap) -> ApiResponseBuilder<T>
where
    T: DeserializeOwned + Send + Sync,
{
    let mut builder = ApiResponse::builder();

    if headers.contains_key("X-Cache") {
        builder = builder.cache(headers["X-Cache"].to_str().unwrap());
    }
    if headers.contains_key("X-Consul-ContentHash") {
        builder = builder.content_hash(headers["X-Consul-ContentHash"].to_str().unwrap())
    }
    if headers.contains_key("X-Consul-Default-ACL-Policy") {
        builder =
            builder.default_acl_policy(headers["X-Consul-Default-ACL-Policy"].to_str().unwrap())
    }
    if headers.contains_key("X-Consul-Index") {
        builder = builder.index(headers["X-Consul-Index"].to_str().unwrap())
    }
    if headers.contains_key("X-Consul-KnownLeader") {
        builder = builder.known_leader(headers["X-Consul-KnownLeader"].to_str().unwrap())
    }
    if headers.contains_key("X-Consul-LastContact") {
        builder = builder.last_contact(headers["X-Consul-LastContact"].to_str().unwrap())
    }
    if headers.contains_key("X-Consul-Query-Backend") {
        builder = builder.query_backend(headers["X-Consul-Query-Backend"].to_str().unwrap())
    }

    builder
}

/// Extracts any API errors found and converts them to [ClientError::APIError].
fn parse_err(e: RestClientError) -> ClientError {
    if let RestClientError::ServerResponseError { code, content } = &e {
        ClientError::APIError {
            code: *code,
            message: content.clone(),
        }
    } else {
        ClientError::from(e)
    }
}

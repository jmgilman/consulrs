use std::str::FromStr;

use rustify::endpoint::{Endpoint, MiddleWare};

/// A [MiddleWare] for adding version and token information to all requests.
///
/// Implements [MiddleWare] to provide support for prepending API version
/// information to all requests and adding an ACL token to the header of all
/// requests. This is automatically passed by the API functions when an endpoint
/// is executed.
#[derive(Debug, Clone)]
pub struct EndpointMiddleware {
    pub token: Option<String>,
    pub version: String,
}
impl MiddleWare for EndpointMiddleware {
    fn request<E: Endpoint>(
        &self,
        _: &E,
        req: &mut http::Request<bytes::Bytes>,
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

        Ok(())
    }

    fn response<E: Endpoint>(
        &self,
        _: &E,
        _: &mut http::Response<bytes::Bytes>,
    ) -> Result<(), rustify::errors::ClientError> {
        Ok(())
    }
}

use async_trait::async_trait;
use derive_builder::Builder;
use rustify::clients::reqwest::Client as HTTPClient;
use std::{env, fs};

use crate::{
    api::{EndpointMiddleware, Features},
    error::ClientError,
};

/// The client interface capabale of interacting with API functions
#[async_trait]
pub trait Client: Send + Sync + Sized {
    /// Returns the underlying HTTP client being used for API calls
    fn http(&self) -> &HTTPClient;

    /// Returns the middleware to be used when executing API calls
    fn middle(&self, features: Option<Features>) -> EndpointMiddleware;

    /// Returns the settings used to configure this client
    fn settings(&self) -> &ConsulClientSettings;
}

/// A client which can be used to execute calls against a Consul server.
///
/// A consul client is configured using [ConsulClientSettings] and will
/// automatically configure a backing instance of a [HTTPClient] which is
/// used for executing [Endpoints][rustify::endpoint::Endpoint].
pub struct ConsulClient {
    pub http: HTTPClient,
    pub settings: ConsulClientSettings,
}

#[async_trait]
impl Client for ConsulClient {
    fn http(&self) -> &HTTPClient {
        &self.http
    }

    fn middle(&self, features: Option<Features>) -> EndpointMiddleware {
        let version_str = format!("v{}", self.settings.version);
        EndpointMiddleware {
            features,
            token: self.settings.token.clone(),
            version: version_str,
        }
    }

    fn settings(&self) -> &ConsulClientSettings {
        &self.settings
    }
}

impl ConsulClient {
    /// Creates a new [ConsulClient] using the given [ConsulClientSettings].
    #[instrument(skip(settings), err)]
    pub fn new(settings: ConsulClientSettings) -> Result<ConsulClient, ClientError> {
        let mut http_client = reqwest::ClientBuilder::new();

        // Disable TLS checks if specified
        if !settings.verify {
            event!(tracing::Level::WARN, "Disabling TLS verification");
        }
        http_client = http_client.danger_accept_invalid_certs(!settings.verify);

        // Adds CA certificates
        for path in &settings.ca_certs {
            let content = std::fs::read(&path).map_err(|e| ClientError::FileReadError {
                source: e,
                path: path.clone(),
            })?;
            let cert = reqwest::Certificate::from_pem(&content).map_err(|e| {
                ClientError::ParseCertificateError {
                    source: e,
                    path: path.clone(),
                }
            })?;

            debug!("Importing CA certificate from {}", path);
            http_client = http_client.add_root_certificate(cert);
        }

        // Add client certificate
        if let (Some(cert), Some(key)) = (&settings.client_cert, &settings.client_key) {
            let cert_content =
                std::fs::read_to_string(&cert).map_err(|e| ClientError::FileReadError {
                    source: e,
                    path: cert.clone(),
                })?;
            let key_content =
                std::fs::read_to_string(&key).map_err(|e| ClientError::FileReadError {
                    source: e,
                    path: key.clone(),
                })?;
            let pem = format!("{}{}", cert_content, key_content);
            let id = reqwest::Identity::from_pem(pem.as_bytes()).map_err(|e| {
                ClientError::ParseCertificateError {
                    source: e,
                    path: cert.clone(),
                }
            })?;

            debug!("Importing client certificate from {}", cert);
            http_client = http_client.identity(id);
        }

        // Configures middleware for endpoints to append API version and token
        debug!("Using API version {}", settings.version);

        let http_client = http_client
            .build()
            .map_err(|e| ClientError::RestClientBuildError { source: e })?;
        let http = HTTPClient::new(settings.address.as_str(), http_client);
        Ok(ConsulClient { settings, http })
    }
}

/// Contains settings for configuring a [ConsulClient].
///
/// Most settings that are not directly configured will have their default value
/// pulled from their respective environment variables. Specifically:
///
/// * `address`: CONSUL_HTTP_ADDR
/// * `ca_certs`: CONSUL_CACERT / CONSUL_CAPATH
/// * `client_cert`: CONSUL_CLIENT_CERT
/// * `client_key`: CONSUL_CLIENT_KEY
/// * `token`: CONSUL_HTTP_TOKEN
/// * `verify`: CONSUL_HTTP_SSL_VERIFY
///
/// Note that the client key must be in an RSA or PKCS#8 format, otherwise the
/// client will fail to be created with a "key not found" error.
#[derive(Builder, Clone, Debug)]
#[builder(setter(into, strip_option))]
pub struct ConsulClientSettings {
    #[builder(default = "self.default_address()")]
    pub address: String,
    #[builder(default = "self.default_ca_certs()")]
    pub ca_certs: Vec<String>,
    #[builder(default = "self.default_client_cert()")]
    pub client_cert: Option<String>,
    #[builder(default = "self.default_client_key()")]
    pub client_key: Option<String>,
    #[builder(setter(into), default = "self.default_token()")]
    pub token: Option<String>,
    #[builder(default = "self.default_verify()")]
    pub verify: bool,
    #[builder(setter(into, strip_option), default = "1")]
    pub version: u8,
}

impl ConsulClientSettingsBuilder {
    fn default_address(&self) -> String {
        match env::var("CONSUL_HTTP_ADDR") {
            Ok(s) => {
                debug!("Using consul address from $CONSUL_HTTP_ADDR");
                s
            }
            Err(_) => {
                debug!("Using default consul address http://127.0.0.1:8500");
                String::from("http://127.0.0.1:8500")
            }
        }
    }

    fn default_ca_certs(&self) -> Vec<String> {
        let mut paths: Vec<String> = Vec::new();

        if let Ok(s) = env::var("CONSUL_CACERT") {
            debug!("Found CA certificate in $CONSUL_CACERT");
            paths.push(s);
        }

        if let Ok(s) = env::var("CONSUL_CAPATH") {
            debug!("Found CA certificate path in $CONSUL_CAPATH");
            if let Ok(p) = fs::read_dir(s) {
                for path in p {
                    paths.push(path.unwrap().path().to_str().unwrap().to_string())
                }
            }
        }

        paths
    }

    fn default_client_cert(&self) -> Option<String> {
        match env::var("CONSUL_CLIENT_CERT") {
            Ok(s) => {
                debug!("Using TLS client certificate from $CONSUL_CLIENT_CERT");
                Some(s)
            }
            Err(_) => {
                debug!("Not using a TLS client certificate");
                None
            }
        }
    }

    fn default_client_key(&self) -> Option<String> {
        match env::var("CONSUL_CLIENT_KEY") {
            Ok(s) => {
                debug!("Using TLS client key from $CONSUL_CLIENT_KEY");
                Some(s)
            }
            Err(_) => {
                debug!("Not using a TLS client key");
                None
            }
        }
    }

    fn default_token(&self) -> Option<String> {
        match env::var("CONSUL_HTTP_TOKEN") {
            Ok(s) => {
                debug!("Using consul ACL token from $CONSUL_HTTP_TOKEN");
                Some(s)
            }
            Err(_) => {
                debug!("Using default empty consul ACL token");
                None
            }
        }
    }

    fn default_verify(&self) -> bool {
        debug!("Checking TLS verification using $CONSUL_HTTP_SSL_VERIFY");
        let verify = env::var("CONSUL_HTTP_SSL_VERIFY").unwrap_or_else(|_| "true".into());
        match verify.as_str() {
            "true" => true,
            "false" => false,
            _ => true,
        }
    }
}

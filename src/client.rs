use async_trait::async_trait;
use derive_builder::Builder;
use rustify::clients::reqwest::Client as HTTPClient;
use std::{env, fs};

use crate::{api::EndpointMiddleware, error::ClientError};

/// The client interface capabale of interacting with API functions
#[async_trait]
pub trait Client: Send + Sync + Sized {
    /// Returns the underlying HTTP client being used for API calls
    fn http(&self) -> &HTTPClient;

    /// Returns the settings used to configure this client
    fn settings(&self) -> &ConsulClientSettings;
}

pub struct ConsulClient {
    pub http: HTTPClient,
    pub middle: EndpointMiddleware,
    pub settings: ConsulClientSettings,
}

#[async_trait]
impl Client for ConsulClient {
    fn http(&self) -> &HTTPClient {
        &self.http
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

            info!("Importing CA certificate from {}", path);
            http_client = http_client.add_root_certificate(cert);
        }

        // Configures middleware for endpoints to append API version and token
        debug!("Using API version {}", settings.version);
        let version_str = format!("v{}", settings.version);
        let middle = EndpointMiddleware {
            token: settings.token.clone(),
            version: version_str,
        };

        let http_client = http_client
            .build()
            .map_err(|e| ClientError::RestClientBuildError { source: e })?;
        let http = HTTPClient::new(settings.address.as_str(), http_client);
        Ok(ConsulClient {
            settings,
            middle,
            http,
        })
    }
}

#[derive(Clone, Debug)]
pub struct ClientCertificate {
    pub data: Vec<u8>,
    pub password: String,
}

#[derive(Builder, Clone, Debug)]
pub struct ConsulClientSettings {
    #[builder(setter(into), default = "self.default_address()")]
    pub address: String,
    #[builder(default = "self.default_ca_certs()")]
    pub ca_certs: Vec<String>,
    pub client_cert: Option<ClientCertificate>,
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
                info!("Using consul address from $CONSUL_HTTP_ADDR");
                s
            }
            Err(_) => {
                info!("Using default consul address http://127.0.0.1:8500");
                String::from("http://127.0.0.1:8500")
            }
        }
    }

    fn default_ca_certs(&self) -> Vec<String> {
        let mut paths: Vec<String> = Vec::new();

        if let Ok(s) = env::var("CONSUL_CACERT") {
            info!("Found CA certificate in $CONSUL_CACERT");
            paths.push(s);
        }

        if let Ok(s) = env::var("CONSUL_CAPATH") {
            info!("Found CA certificate path in $CONSUL_CAPATH");
            if let Ok(p) = fs::read_dir(s) {
                for path in p {
                    paths.push(path.unwrap().path().to_str().unwrap().to_string())
                }
            }
        }

        paths
    }

    fn default_token(&self) -> Option<String> {
        match env::var("CONSUL_HTTP_TOKEN") {
            Ok(s) => {
                info!("Using consul ACL token from $CONSUL_HTTP_TOKEN");
                Some(s)
            }
            Err(_) => {
                info!("Using default empty consul ACL token");
                None
            }
        }
    }

    fn default_verify(&self) -> bool {
        info!("Checking TLS verification using $CONSUL_HTTP_SSL_VERIFY");
        let verify = env::var("CONSUL_HTTP_SSL_VERIFY").unwrap_or("true".into());
        match verify.as_str() {
            "true" => true,
            "false" => false,
            _ => true,
        }
    }
}

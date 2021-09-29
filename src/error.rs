use thiserror::Error;

/// The common error type returned by this crate
#[derive(Error, Debug)]
pub enum ClientError {
    #[error("The Consul server returned an error (status code {code})")]
    APIError { code: u16 },
    #[error("Failed decoding Base64 response")]
    Base64DecodeError { source: base64::DecodeError },
    #[error("Error reading file: {path}")]
    FileReadError {
        source: std::io::Error,
        path: String,
    },
    #[error("Error parsing CA certificate as PEM encoded certificate: {path}")]
    ParseCertificateError {
        source: reqwest::Error,
        path: String,
    },
    #[error("The request returned an empty response")]
    ResponseEmptyError,
    #[error("An error occurred with the request")]
    RestClientError {
        #[from]
        source: rustify::errors::ClientError,
    },
    #[error("Error configuring REST client")]
    RestClientBuildError { source: reqwest::Error },
}

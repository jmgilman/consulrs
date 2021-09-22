use thiserror::Error;

/// The common error type returned by this crate
#[derive(Error, Debug)]
pub enum ClientError {
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
    #[error("Error configuring REST client")]
    RestClientBuildError { source: reqwest::Error },
}

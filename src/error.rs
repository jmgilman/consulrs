use std::str::Utf8Error;

use thiserror::Error;

/// The common error type returned by this crate
#[derive(Error, Debug)]
pub enum ClientError {
    #[error("The Consul server returned an error (status code {code})")]
    APIError { code: u16, message: Option<String> },
    #[error("Failed decoding Base64 response")]
    Base64DecodeError { source: base64::DecodeError },
    #[error("Empty response")]
    EmptyResponseError,
    #[error("Error reading file: {path}")]
    FileReadError {
        source: std::io::Error,
        path: String,
    },
    #[error("Error deserializing JSON string")]
    JsonDeserializeError { source: serde_json::Error },
    #[error("Error Serializing JSON string")]
    JsonSerializeError { source: serde_json::Error },
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
    #[error("Error decoding bytes into UTF-8 string")]
    Utf8DecodeError { source: Utf8Error },
}

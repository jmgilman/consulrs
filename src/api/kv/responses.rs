use crate::error::ClientError;
use serde::{Deserialize, Serialize};

/// Response from executing
/// [ReadKeyRequest][crate::api::kv::requests::ReadKeyRequest]
#[derive(Deserialize, Debug, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct ReadKeyResponse {
    pub create_index: u64,
    pub modify_index: u64,
    pub lock_index: u64,
    pub key: String,
    pub flags: u64,
    pub session: Option<String>,
    pub value: String,
}

impl ReadKeyResponse {
    pub fn value(&self) -> Result<Vec<u8>, ClientError> {
        base64::decode(&self.value).map_err(|e| ClientError::Base64DecodeError { source: e })
    }
}

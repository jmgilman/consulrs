use std::convert::TryInto;

use derive_builder::Builder;
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use serde_with::skip_serializing_none;

use crate::error::ClientError;

/// A string containing Base64 encoded content.
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct Base64String(String);

impl TryInto<Vec<u8>> for Base64String {
    type Error = ClientError;

    fn try_into(self) -> Result<Vec<u8>, Self::Error> {
        base64::Engine::decode(&base64::engine::general_purpose::STANDARD, self.0)
            .map_err(|e| ClientError::Base64DecodeError { source: e })
    }
}

impl TryInto<String> for Base64String {
    type Error = ClientError;

    fn try_into(self) -> Result<String, Self::Error> {
        let bytes: Vec<u8> = self.try_into()?;
        std::str::from_utf8(&bytes)
            .map_err(|e| ClientError::Utf8DecodeError { source: e })
            .map(|s| s.into())
    }
}

#[skip_serializing_none]
#[derive(Builder, Clone, Debug, Default, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
#[builder(setter(into, strip_option), default)]
pub struct KVPair {
    pub create_index: u64,
    pub flags: u64,
    pub key: String,
    pub lock_index: u64,
    pub modify_index: u64,
    pub namespace: Option<String>,
    pub session: Option<String>,
    pub value: Option<Base64String>,
}

#[derive(Debug)]
pub struct GenericKVPair<T: DeserializeOwned> {
    pub create_index: u64,
    pub flags: u64,
    pub key: String,
    pub lock_index: u64,
    pub modify_index: u64,
    pub namespace: Option<String>,
    pub session: Option<String>,
    pub value: T,
}

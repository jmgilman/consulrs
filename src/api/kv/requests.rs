use crate::api::Features;

use super::common::KVPair;
use consulrs_derive::QueryEndpoint;
use derive_builder::Builder;
use rustify_derive::Endpoint;
use serde::Serialize;
use std::fmt::Debug;

/// ## Read Key
/// This endpoint returns the specified key.
///
/// * Path: kv/{self.key}
/// * Method: GET
/// * Response: [ReadKeyResponse]
/// * Reference: https://www.consul.io/api-docs/kv#read-key
#[derive(Builder, Debug, Default, Endpoint, QueryEndpoint)]
#[endpoint(path = "kv/{self.key}", response = "Vec<KVPair>", builder = "true")]
#[builder(setter(into, strip_option), default)]
pub struct ReadKeyRequest {
    #[endpoint(skip)]
    pub features: Option<Features>,
    #[endpoint(skip)]
    pub key: String,
    #[endpoint(query)]
    pub dc: Option<String>,
    #[endpoint(query)]
    pub ns: Option<String>,
    #[endpoint(query)]
    pub recurse: Option<bool>,
}

/// ## Read Key
/// This endpoint returns the raw value of the specified key.
///
/// * Path: kv/{self.key}
/// * Method: GET
/// * Response: [ReadKeyResponse]
/// * Reference: https://www.consul.io/api-docs/kv#read-key
#[derive(Builder, Debug, Default, Endpoint, QueryEndpoint)]
#[endpoint(path = "kv/{self.key}", response = "Vec<u8>", builder = "true")]
#[builder(setter(into, strip_option), default)]
pub struct ReadRawKeyRequest {
    #[endpoint(skip)]
    pub features: Option<Features>,
    #[endpoint(skip)]
    pub key: String,
    #[endpoint(query)]
    pub dc: Option<String>,
    #[endpoint(query)]
    pub ns: Option<String>,
    #[endpoint(query)]
    #[builder(setter(skip), default = "true")]
    pub raw: bool,
}

/// ## Read Keys
/// This endpoint returns a list of key names at the specified key.
///
/// * Path: kv/{self.key}
/// * Method: GET
/// * Response: [Vec<String>]
/// * Reference: https://www.consul.io/api-docs/kv#read-key
#[derive(Builder, Debug, Default, Endpoint, QueryEndpoint)]
#[endpoint(path = "kv/{self.key}", response = "Vec<String>", builder = "true")]
#[builder(setter(into, strip_option), default)]
pub struct ReadKeysRequest {
    #[endpoint(skip)]
    pub features: Option<Features>,
    #[endpoint(skip)]
    pub key: String,
    #[endpoint(query)]
    pub dc: Option<String>,
    #[endpoint(query)]
    #[builder(setter(skip), default = "true")]
    pub keys: bool,
    #[endpoint(query)]
    pub ns: Option<String>,
    #[endpoint(query)]
    pub raw: Option<bool>,
    #[endpoint(query)]
    pub recurse: Option<bool>,
    #[endpoint(query)]
    pub separator: Option<String>, // Only valid when `keys` is set
}

/// ## Create/Update Key
/// This endpoint updates the value of the specified key.
///
/// * Path: kv/{self.key}
/// * Method: PUT
/// * Response: [bool]
/// * Reference: https://www.consul.io/api-docs/kv#create-update-key
#[derive(Builder, Debug, Default, Endpoint, QueryEndpoint)]
#[endpoint(
    path = "kv/{self.key}",
    method = "PUT",
    response = "bool",
    builder = "true"
)]
#[builder(setter(into, strip_option), default)]
pub struct SetKeyRequest {
    #[endpoint(skip)]
    pub features: Option<Features>,
    #[endpoint(skip)]
    pub key: String,
    #[endpoint(raw)]
    pub value: Vec<u8>,
    #[endpoint(query)]
    pub acquire: Option<String>,
    #[endpoint(query)]
    pub cas: Option<u64>,
    #[endpoint(query)]
    pub dc: Option<String>,
    #[endpoint(query)]
    pub flags: Option<u64>,
    #[endpoint(query)]
    pub ns: Option<String>,
    #[endpoint(query)]
    pub release: Option<String>,
}

/// ## Delete Key
/// This endpoint deletes a single key or all keys sharing a prefix.
///
/// * Path: kv/{self.key}
/// * Method: DELETE
/// * Response: [bool]
/// * Reference: https://www.consul.io/api-docs/kv#delete-key
#[derive(Builder, Debug, Default, Endpoint, QueryEndpoint)]
#[endpoint(
    path = "kv/{self.key}",
    method = "DELETE",
    response = "bool",
    builder = "true"
)]
#[builder(setter(into, strip_option), default)]
pub struct DeleteKeyRequest {
    #[endpoint(skip)]
    pub features: Option<Features>,
    #[endpoint(skip)]
    pub key: String,
    #[endpoint(query)]
    pub cas: Option<u64>,
    #[endpoint(query)]
    pub dc: Option<String>,
    #[endpoint(query)]
    pub ns: Option<String>,
    #[endpoint(query)]
    pub recurse: Option<bool>,
}

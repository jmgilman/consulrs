use std::collections::HashMap;

use serde::Deserialize;

use crate::api::{check::responses::CheckResponse, connect::responses::ProxyResponse};

#[derive(Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct ConnectResponse {
    pub native: Option<bool>,
    pub sidecar_service: Option<SidecarServiceResponse>,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct ServiceResponse {
    pub address: Option<String>,
    pub check: Option<CheckResponse>,
    pub checks: Option<Vec<CheckResponse>>,
    pub connect: Option<ConnectResponse>,
    pub enable_tag_override: Option<bool>,
    #[serde(rename = "ID")]
    pub id: Option<String>,
    pub kind: Option<String>,
    pub meta: Option<HashMap<String, String>>,
    pub name: Option<String>,
    pub ns: Option<String>,
    pub port: Option<u64>,
    pub proxy: Option<ProxyResponse>,
    pub tagged_addresses: Option<HashMap<String, String>>,
    pub tags: Option<Vec<String>>,
    pub weights: Option<WeightsResponse>,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct SidecarServiceResponse {
    pub address: Option<String>,
    pub check: Option<CheckResponse>,
    pub checks: Option<Vec<CheckResponse>>,
    pub enable_tag_override: Option<bool>,
    #[serde(rename = "ID")]
    pub id: Option<String>,
    pub kind: Option<String>,
    pub meta: Option<HashMap<String, String>>,
    pub name: String,
    pub ns: Option<String>,
    pub port: Option<u64>,
    pub proxy: Option<ProxyResponse>,
    pub tagged_addresses: Option<HashMap<String, String>>,
    pub tags: Option<Vec<String>>,
    pub weights: Option<WeightsResponse>,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct WeightsResponse {
    pub passing: u64,
    pub warning: u64,
}

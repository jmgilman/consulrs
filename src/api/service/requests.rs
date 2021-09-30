use crate::api::{check::requests::RegisterCheckRequest, Features};
use consulrs_derive::QueryEndpoint;
use derive_builder::Builder;
use rustify_derive::Endpoint;
use serde::Serialize;
use std::{collections::HashMap, fmt::Debug};

/// ## Register Service
/// This endpoint adds a new service, with optional health checks, to the local
/// agent.
///
/// * Path: agent/service/register
/// * Method: PUT
/// * Response: N/A
/// * Reference: https://www.consul.io/api-docs/agent/service#register-service
#[derive(Builder, Clone, Debug, Default, Endpoint, QueryEndpoint, Serialize)]
#[endpoint(path = "agent/service/register", method = "PUT", builder = "true")]
#[serde(rename_all = "PascalCase")]
#[builder(setter(into, strip_option), default)]
pub struct RegisterServiceRequest {
    #[endpoint(skip)]
    #[serde(skip)]
    pub features: Option<Features>,
    pub name: String,
    pub address: Option<String>,
    pub check: Option<RegisterCheckRequest>,
    pub checks: Option<Vec<RegisterCheckRequest>>,
    pub connect: Option<Connect>,
    pub enable_tag_override: Option<bool>,
    #[serde(rename = "ID")]
    pub id: Option<String>,
    pub kind: Option<String>,
    pub meta: Option<HashMap<String, String>>,
    pub ns: Option<String>,
    pub port: Option<u64>,
    pub proxy: Option<Proxy>,
    pub tagged_addresses: Option<HashMap<String, String>>,
    pub tags: Option<Vec<String>>,
    pub weights: Option<Weight>,
}

#[derive(Builder, Clone, Debug, Default, Serialize)]
#[serde(rename_all = "PascalCase")]
#[builder(setter(into, strip_option), default)]
pub struct Connect {
    pub native: Option<bool>,
    pub sidecar_service: Option<SidecarService>,
}

#[derive(Clone, Debug, Serialize)]
pub struct Proxy {
    pub destination_service_name: String,
}

#[derive(Builder, Clone, Debug, Default, Serialize)]
#[builder(setter(into, strip_option), default)]
#[serde(rename_all = "PascalCase")]
pub struct Weight {
    pub passing: Option<String>,
    pub warning: Option<String>,
}

#[derive(Builder, Clone, Debug, Default, Serialize)]
#[serde(rename_all = "PascalCase")]
#[builder(setter(into, strip_option), default)]
pub struct SidecarService {
    pub name: String,
    pub address: Option<String>,
    pub check: Option<RegisterCheckRequest>,
    pub checks: Option<Vec<RegisterCheckRequest>>,
    pub enable_tag_override: Option<bool>,
    #[serde(rename = "ID")]
    pub id: Option<String>,
    pub kind: Option<String>,
    pub meta: Option<HashMap<String, String>>,
    pub ns: Option<String>,
    pub port: Option<u64>,
    pub proxy: Option<Proxy>,
    pub tagged_addresses: Option<HashMap<String, String>>,
    pub tags: Option<Vec<String>>,
    pub weights: Option<Weight>,
}

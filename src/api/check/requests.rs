use crate::api::Features;

use consulrs_derive::QueryEndpoint;
use derive_builder::Builder;
use rustify_derive::Endpoint;
use serde::Serialize;
use serde_with::skip_serializing_none;
use std::{collections::HashMap, fmt::Debug};

/// ## Register Check
/// This endpoint adds a new service, with optional health checks, to the local
/// agent.
///
/// * Path: agent/check/register
/// * Method: PUT
/// * Response: N/A
/// * Reference: https://www.consul.io/api-docs/agent/check#register-check
#[derive(Builder, Clone, Debug, Default, Endpoint, QueryEndpoint, Serialize)]
#[endpoint(path = "agent/check/register", method = "PUT", builder = "true")]
#[serde(rename_all = "PascalCase")]
#[builder(setter(into, strip_option), default)]
pub struct RegisterCheckRequest {
    #[endpoint(skip)]
    #[serde(skip)]
    pub features: Option<Features>,
    pub name: String,
    pub alias_node: Option<String>,
    pub alias_service: Option<String>,
    pub args: Option<Vec<String>>,
    pub body: Option<String>,
    pub deregister_critical_service_after: Option<String>,
    pub docker_container_id: Option<String>,
    pub failures_before_critical: Option<u64>,
    #[serde(rename = "GRPC")]
    pub grpc: Option<String>,
    #[serde(rename = "GRPCUseTLS")]
    pub grpc_use_tls: Option<bool>,
    #[serde(rename = "H2PING")]
    pub h2_ping: Option<String>,
    pub header: Option<HashMap<String, String>>,
    #[serde(rename = "HTTP")]
    pub http: Option<String>,
    #[serde(rename = "ID")]
    pub id: Option<String>,
    pub interval: Option<String>,
    pub method: Option<String>,
    pub namespace: Option<String>,
    pub notes: Option<String>,
    pub output_max_size: Option<u64>,
    pub service_id: Option<String>,
    pub status: Option<String>,
    pub success_before_passing: Option<u64>,
    #[serde(rename = "TCP")]
    pub tcp: Option<String>,
    pub timeout: Option<String>,
    #[serde(rename = "TLSServerName")]
    pub tls_server_name: Option<String>,
    #[serde(rename = "TLSSkipVerify")]
    pub tls_skip_verify: Option<String>,
    #[serde(rename = "TTL")]
    pub ttl: Option<String>,
}

#[skip_serializing_none]
#[derive(Builder, Clone, Debug, Default, Serialize)]
#[serde(rename_all = "PascalCase")]
#[builder(setter(into, strip_option), default)]
pub struct CheckRequest {
    #[serde(rename = "CheckID")]
    pub check_id: Option<String>,
    pub definition: Option<String>,
    pub exposed_port: Option<u64>,
    #[serde(rename = "ID")]
    pub id: Option<String>,
    pub name: String,
    pub namespace: Option<String>,
    pub node: Option<String>,
    pub notes: Option<String>,
    pub output: Option<String>,
    #[serde(rename = "ServiceID")]
    pub service_id: Option<String>,
    pub service_name: Option<String>,
    pub status: Option<String>,
    #[serde(rename = "Type")]
    pub ty: Option<String>,
}

#[skip_serializing_none]
#[derive(Builder, Clone, Debug, Default, Serialize)]
#[serde(rename_all = "PascalCase")]
#[builder(setter(into, strip_option), default)]
pub struct CheckDefinitionRequest {
    pub body: Option<String>,
    pub deregister_critical_service_after_duration: Option<String>,
    pub header: Option<HashMap<String, String>>,
    #[serde(rename = "HTTP")]
    pub http: Option<String>,
    pub interval_duration: Option<String>,
    pub method: Option<String>,
    pub timeout_duration: Option<String>,
    #[serde(rename = "TLSServerName")]
    pub tls_server_name: Option<String>,
    #[serde(rename = "TLSSkipVerify")]
    pub tls_skip_verify: Option<bool>,
    #[serde(rename = "TCP")]
    pub tcp: Option<String>,
}

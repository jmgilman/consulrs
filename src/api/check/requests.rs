use crate::api::Features;

use super::responses::AgentCheckResponse;
use consulrs_derive::QueryEndpoint;
use derive_builder::Builder;
use rustify_derive::Endpoint;
use serde::Serialize;
use serde_with::skip_serializing_none;
use std::{collections::HashMap, fmt::Debug};

/// ## List Checks
/// This endpoint returns all checks that are registered with the local agent.
///
/// * Path: agent/checks
/// * Method: GET
/// * Response: [HashMap<String, AgentCheckResponse>]
/// * Reference: https://www.consul.io/api-docs/catalog#list-services-for-node
#[derive(Builder, Debug, Default, Endpoint, QueryEndpoint)]
#[endpoint(
    path = "agent/checks",
    response = "HashMap<String, AgentCheckResponse>",
    builder = "true"
)]
#[builder(setter(into, strip_option), default)]
pub struct ListChecksRequest {
    #[endpoint(skip)]
    pub features: Option<Features>,
    #[endpoint(query)]
    pub ns: Option<String>,
}

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

/// ## Deregister Check
/// This endpoint remove a check from the local agent.
///
/// * Path: agent/check/deregister/{self.check}
/// * Method: PUT
/// * Response: N/A
/// * Reference: https://www.consul.io/api-docs/agent/check#deregister-check
#[derive(Builder, Debug, Default, Endpoint, QueryEndpoint)]
#[endpoint(
    path = "agent/check/deregister/{self.check}",
    method = "PUT",
    builder = "true"
)]
#[builder(setter(into, strip_option), default)]
pub struct DeregisterCheckRequest {
    #[endpoint(skip)]
    pub features: Option<Features>,
    #[endpoint(skip)]
    pub check: String,
    #[endpoint(query)]
    pub ns: Option<String>,
}

/// ## TTL Check Pass
/// This endpoint is used with a TTL type check to set the status of the check
/// to passing and to reset the TTL clock.
///
/// * Path: agent/check/pass/{self.check}
/// * Method: PUT
/// * Response: N/A
/// * Reference: https://www.consul.io/api-docs/agent/check#ttl-check-pass
#[derive(Builder, Debug, Default, Endpoint, QueryEndpoint)]
#[endpoint(
    path = "agent/check/pass/{self.check}",
    method = "PUT",
    builder = "true"
)]
#[builder(setter(into, strip_option), default)]
pub struct TtlCheckPassRequest {
    #[endpoint(skip)]
    pub features: Option<Features>,
    #[endpoint(skip)]
    pub check: String,
    pub note: Option<String>,
    #[endpoint(query)]
    pub ns: Option<String>,
}

/// ## TTL Check Warn
/// This endpoint is used with a TTL type check to set the status of the check
/// to warning and to reset the TTL clock.
///
/// * Path: agent/check/warn/{self.check}
/// * Method: PUT
/// * Response: N/A
/// * Reference: https://www.consul.io/api-docs/agent/check#ttl-check-warn
#[derive(Builder, Debug, Default, Endpoint, QueryEndpoint)]
#[endpoint(
    path = "agent/check/warn/{self.check}",
    method = "PUT",
    builder = "true"
)]
#[builder(setter(into, strip_option), default)]
pub struct TtlCheckWarnRequest {
    #[endpoint(skip)]
    pub features: Option<Features>,
    #[endpoint(skip)]
    pub check: String,
    pub note: Option<String>,
    #[endpoint(query)]
    pub ns: Option<String>,
}

/// ## TTL Check Fail
/// This endpoint is used with a TTL type check to set the status of the check
/// to critical and to reset the TTL clock.
///
/// * Path: agent/check/fail/{self.check}
/// * Method: PUT
/// * Response: N/A
/// * Reference: https://www.consul.io/api-docs/agent/check#ttl-check-fail
#[derive(Builder, Debug, Default, Endpoint, QueryEndpoint)]
#[endpoint(
    path = "agent/check/fail/{self.check}",
    method = "PUT",
    builder = "true"
)]
#[builder(setter(into, strip_option), default)]
pub struct TtlCheckFailRequest {
    #[endpoint(skip)]
    pub features: Option<Features>,
    #[endpoint(skip)]
    pub check: String,
    pub note: Option<String>,
    #[endpoint(query)]
    pub ns: Option<String>,
}

/// ## TTL Check Update
/// This endpoint is used with a TTL type check to set the status of the check
/// and to reset the TTL clock.
///
/// * Path: agent/check/update/{self.check}
/// * Method: PUT
/// * Response: N/A
/// * Reference: https://www.consul.io/api-docs/agent/check#ttl-check-update
#[derive(Builder, Debug, Default, Endpoint, QueryEndpoint, Serialize)]
#[endpoint(
    path = "agent/check/update/{self.check}",
    method = "PUT",
    builder = "true"
)]
#[builder(setter(into, strip_option), default)]
pub struct TtlCheckUpdateRequest {
    #[endpoint(skip)]
    #[serde(skip)]
    pub features: Option<Features>,
    #[endpoint(skip)]
    pub check: String,
    #[endpoint(query)]
    pub ns: Option<String>,
    #[serde(rename = "Output")]
    pub output: Option<String>,
    #[serde(rename = "Status")]
    pub status: Option<String>,
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

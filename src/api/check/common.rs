use derive_builder::Builder;
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;
use std::{collections::HashMap, fmt::Debug};

#[skip_serializing_none]
#[derive(Builder, Clone, Debug, Default, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
#[builder(setter(into, strip_option), default)]
pub struct AgentCheck {
    #[serde(rename = "CheckID")]
    pub check_id: Option<String>,
    pub create_index: Option<u64>,
    pub definition: Option<HealthCheckDefinition>,
    pub exposed_port: Option<u64>,
    pub modify_index: Option<u64>,
    pub name: Option<String>,
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
#[derive(Builder, Clone, Debug, Default, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
#[builder(setter(into, strip_option), default)]
pub struct AgentServiceCheck {
    pub alias_node: Option<String>,
    pub alias_service: Option<String>,
    pub args: Option<Vec<String>>,
    pub body: Option<String>,
    #[serde(rename = "CheckID")]
    pub check_id: Option<String>,
    pub deregister_critical_service_after: Option<String>,
    #[serde(rename = "DockerContainerID")]
    pub docker_container_id: Option<String>,
    pub failures_before_critical: Option<u64>,
    #[serde(rename = "GRPC")]
    pub grpc: Option<String>,
    #[serde(rename = "GRPCUseTLS")]
    pub grpc_use_tls: Option<bool>,
    pub header: Option<HashMap<String, String>>,
    #[serde(rename = "HTTP")]
    pub http: Option<String>,
    pub interval: Option<String>,
    pub method: Option<String>,
    pub name: Option<String>,
    pub notes: Option<String>,
    pub shell: Option<String>,
    pub status: Option<String>,
    pub success_before_passing: Option<u64>,
    #[serde(rename = "TCP")]
    pub tcp: Option<String>,
    pub timeout: Option<String>,
    #[serde(rename = "TLSServerName")]
    pub tls_server_name: Option<String>,
    #[serde(rename = "TLSSkipVerify")]
    pub tlk_skip_verify: Option<String>,
    #[serde(rename = "TTL")]
    pub ttl: Option<String>,
}

#[skip_serializing_none]
#[derive(Builder, Clone, Debug, Default, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
#[builder(setter(into, strip_option), default)]
pub struct HealthCheck {
    #[serde(rename = "CheckID")]
    pub check_id: Option<String>,
    pub create_index: Option<u64>,
    pub definition: Option<HealthCheckDefinition>,
    pub modify_index: Option<u64>,
    pub name: Option<String>,
    pub namespace: Option<String>,
    pub node: Option<String>,
    pub notes: Option<String>,
    pub output: Option<String>,
    #[serde(rename = "ServiceID")]
    pub service_id: Option<String>,
    pub service_name: Option<String>,
    pub service_tags: Option<Vec<String>>,
    pub status: Option<String>,
    #[serde(rename = "Type")]
    pub ty: Option<String>,
}

#[skip_serializing_none]
#[derive(Builder, Clone, Debug, Default, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
#[builder(setter(into, strip_option), default)]
pub struct HealthCheckDefinition {
    pub body: Option<String>,
    pub deregister_critical_service_after_duration: Option<String>,
    pub header: Option<HashMap<String, String>>,
    #[serde(rename = "HTTP")]
    pub http: Option<String>,
    pub interval_duration: Option<String>,
    pub method: Option<String>,
    #[serde(rename = "TCP")]
    pub tcp: Option<String>,
    pub timeout_duration: Option<String>,
    #[serde(rename = "TLSServerName")]
    pub tls_server_name: Option<String>,
    #[serde(rename = "TLSSkipVerify")]
    pub tls_skip_verify: Option<bool>,
}

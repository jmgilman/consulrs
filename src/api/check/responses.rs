use std::collections::HashMap;

use serde::Deserialize;

#[derive(Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct CheckResponse {
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
    pub name: String,
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

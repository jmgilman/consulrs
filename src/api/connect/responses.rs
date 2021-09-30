use serde::Deserialize;
use std::collections::HashMap;

#[derive(Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct ProxyResponse {
    pub config: Option<HashMap<String, String>>,
    pub destination_service_name: Option<String>,
    pub destination_service_id: Option<String>,
    pub expose: Option<ExposeResponse>,
    pub local_service_address: Option<String>,
    pub local_service_port: Option<u64>,
    pub mesh_gateway: Option<MeshGatewayResponse>,
    pub mode: Option<String>,
    pub transparent_proxy: Option<TransparentProxyResponse>,
    pub upstreams: Option<Vec<UpstreamResponse>>,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct TransparentProxyResponse {
    pub outbound_listener_port: Option<u64>,
    pub dialed_directly: Option<bool>,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct UpstreamResponse {
    pub centrally_configured: Option<bool>,
    pub config: Option<HashMap<String, String>>,
    pub datacenter: Option<String>,
    pub destination_type: Option<String>,
    pub destination_namespace: Option<String>,
    pub destination_name: String,
    pub local_bind_address: Option<String>,
    pub local_bind_port: Option<u64>,
    pub local_bind_socket_path: Option<String>,
    pub local_bind_socket_mode: Option<String>,
    pub mesh_gateway: Option<MeshGatewayResponse>,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct MeshGatewayResponse {
    mode: Option<String>,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct ExposeResponse {
    checks: Option<bool>,
    paths: Option<Vec<ExposePathResponse>>,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct ExposePathResponse {
    pub listener_port: Option<u64>,
    pub local_path_port: Option<u64>,
    pub path: Option<String>,
    pub parsed_from_check: bool,
    pub protocol: Option<String>,
}

use derive_builder::Builder;
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;
use std::{collections::HashMap, fmt::Debug};

#[skip_serializing_none]
#[derive(Builder, Clone, Debug, Default, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
#[builder(setter(into, strip_option), default)]
pub struct ExposeConfig {
    pub checks: Option<bool>,
    pub paths: Option<Vec<ExposePath>>,
}

#[skip_serializing_none]
#[derive(Builder, Clone, Debug, Default, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
#[builder(setter(into, strip_option), default)]
pub struct ExposePath {
    pub listener_port: Option<u64>,
    pub local_path_port: Option<u64>,
    pub path: Option<String>,
    pub parsed_from_check: Option<bool>,
    pub protocol: Option<String>,
}

#[skip_serializing_none]
#[derive(Builder, Clone, Debug, Default, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
#[builder(setter(into, strip_option), default)]
pub struct MeshGatewayConfig {
    pub mode: Option<String>,
}

#[skip_serializing_none]
#[derive(Builder, Clone, Debug, Default, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
#[builder(setter(into, strip_option), default)]
pub struct TransparentProxyConfig {
    pub dialed_directly: Option<bool>,
    pub outbound_listener_port: Option<u64>,
}

#[skip_serializing_none]
#[derive(Builder, Clone, Debug, Default, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
#[builder(setter(into, strip_option), default)]
pub struct Upstream {
    pub centrally_configured: Option<bool>,
    pub config: Option<HashMap<String, String>>,
    pub datacenter: Option<String>,
    pub destination_name: Option<String>,
    pub destination_namespace: Option<String>,
    pub destination_type: Option<String>,
    pub local_bind_address: Option<String>,
    pub local_bind_port: Option<u64>,
    pub local_bind_socket_mode: Option<String>,
    pub local_bind_socket_path: Option<String>,
    pub mesh_gateway: Option<MeshGatewayConfig>,
}

use derive_builder::Builder;
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;
use std::{collections::HashMap, fmt::Debug};

use crate::api::{
    check::common::{AgentServiceCheck, HealthCheck},
    connect::common::{ExposeConfig, MeshGatewayConfig, TransparentProxyConfig, Upstream},
};

#[skip_serializing_none]
#[derive(Builder, Clone, Debug, Default, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
#[builder(setter(into, strip_option), default)]
pub struct AgentService {
    pub address: Option<String>,
    pub connect: Option<AgentServiceConnect>,
    pub content_hash: Option<String>,
    pub create_index: Option<u64>,
    pub datacenter: Option<String>,
    pub enable_tag_override: Option<bool>,
    #[serde(rename = "ID")]
    pub id: Option<String>,
    pub kind: Option<String>,
    pub meta: Option<HashMap<String, String>>,
    pub modify_index: Option<u64>,
    pub name: Option<String>,
    pub namespace: Option<String>,
    pub port: Option<u64>,
    pub proxy: Option<AgentServiceConnectProxy>,
    pub service: Option<String>,
    pub socket_path: Option<String>,
    pub tagged_addresses: Option<HashMap<String, AgentServiceTaggedAddress>>,
    pub tags: Option<Vec<String>>,
    pub weights: Option<AgentWeights>,
}

#[skip_serializing_none]
#[derive(Builder, Clone, Debug, Default, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
#[builder(setter(into, strip_option), default)]
pub struct AgentServiceTaggedAddress {
    pub address: Option<String>,
    pub port: Option<u64>,
}

#[skip_serializing_none]
#[derive(Builder, Clone, Debug, Default, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
#[builder(setter(into, strip_option), default)]
pub struct AgentServiceChecksInfo {
    pub aggregated_status: String,
    pub checks: Vec<HealthCheck>,
    pub service: AgentService,
}

#[skip_serializing_none]
#[derive(Builder, Clone, Debug, Default, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
#[builder(setter(into, strip_option), default)]
pub struct AgentServiceConnectProxy {
    pub config: Option<HashMap<String, String>>,
    #[serde(rename = "DestinationServiceID")]
    pub destination_service_id: Option<String>,
    pub destination_service_name: Option<String>,
    pub expose: Option<ExposeConfig>,
    pub local_service_address: Option<String>,
    pub local_service_port: Option<u64>,
    pub local_service_socket_path: Option<String>,
    pub mesh_gateway: Option<MeshGatewayConfig>,
    pub mode: Option<String>,
    pub transparent_proxy: Option<TransparentProxyConfig>,
    pub upstreams: Option<Vec<Upstream>>,
}

#[skip_serializing_none]
#[derive(Builder, Clone, Debug, Default, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
#[builder(setter(into, strip_option), default)]
pub struct AgentServiceConnect {
    pub native: Option<bool>,
    pub sidecar_service: Option<AgentServiceRegistration>,
}

#[skip_serializing_none]
#[derive(Builder, Clone, Debug, Default, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
#[builder(setter(into, strip_option), default)]
pub struct AgentServiceRegistration {
    pub address: Option<String>,
    pub check: Option<AgentServiceCheck>,
    pub checks: Option<Vec<AgentServiceCheck>>,
    pub connect: Option<Box<AgentServiceConnect>>,
    pub enable_tag_override: Option<bool>,
    #[serde(rename = "ID")]
    pub id: Option<String>,
    pub kind: Option<String>,
    pub meta: Option<HashMap<String, String>>,
    pub name: Option<String>,
    pub ns: Option<String>,
    pub port: Option<u64>,
    pub proxy: Option<AgentServiceConnectProxy>,
    pub tagged_addresses: Option<HashMap<String, String>>,
    pub tags: Option<Vec<String>>,
    pub weights: Option<AgentWeights>,
}

#[skip_serializing_none]
#[derive(Builder, Clone, Debug, Default, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
#[builder(setter(into, strip_option), default)]
pub struct AgentWeights {
    pub passing: Option<u64>,
    pub warning: Option<u64>,
}

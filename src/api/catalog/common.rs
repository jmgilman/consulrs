use crate::api::service::common::{AgentServiceConnect, AgentServiceConnectProxy, AgentWeights};
use derive_builder::Builder;
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;
use std::{collections::HashMap, fmt::Debug};

#[skip_serializing_none]
#[derive(Builder, Clone, Debug, Default, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
#[builder(setter(into, strip_option), default)]
pub struct CatalogService {
    pub address: Option<String>,
    pub create_index: Option<u64>,
    pub datacenter: Option<String>,
    #[serde(rename = "ID")]
    pub id: Option<String>,
    pub node_meta: Option<HashMap<String, String>>,
    pub modify_index: Option<u64>,
    pub namespace: Option<String>,
    pub node: Option<String>,
    pub service_address: Option<String>,
    pub service_connect: Option<AgentServiceConnect>,
    pub service_enable_tag_override: Option<bool>,
    #[serde(rename = "ServiceID")]
    pub service_id: Option<String>,
    pub service_kind: Option<String>,
    pub service_meta: Option<HashMap<String, String>>,
    pub service_name: Option<String>,
    pub service_port: Option<u64>,
    pub service_proxy: Option<AgentServiceConnectProxy>,
    pub service_socket_path: Option<String>,
    pub service_tags: Option<Vec<String>>,
    pub service_weights: Option<AgentWeights>,
    pub tagged_addresses: Option<HashMap<String, String>>,
}

#[skip_serializing_none]
#[derive(Builder, Clone, Debug, Default, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
#[builder(setter(into, strip_option), default)]
pub struct Node {
    pub address: String,
    pub create_index: u64,
    pub datacenter: String,
    #[serde(rename = "ID")]
    pub id: String,
    pub meta: Option<HashMap<String, String>>,
    pub modify_index: u64,
    pub node: String,
    pub tagged_addresses: Option<HashMap<String, String>>,
}

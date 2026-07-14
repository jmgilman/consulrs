use derive_builder::Builder;
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::Debug;

#[skip_serializing_none]
#[derive(Builder, Clone, Debug, Default, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
#[builder(setter(into, strip_option), default)]
pub struct AgentConfiguration {
    pub config: Option<AgentConfig>,
}

#[skip_serializing_none]
#[derive(Builder, Clone, Debug, Default, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
#[builder(setter(into, strip_option), default)]
pub struct AgentConfig {
    pub datacenter: Option<String>,
    pub node_name: Option<String>,
    #[serde(rename = "NodeID")]
    pub node_id: Option<String>,
    pub server: Option<bool>,
    pub revision: Option<String>,
    pub version: Option<String>,
}

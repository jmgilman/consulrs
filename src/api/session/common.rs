use derive_builder::Builder;
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

#[skip_serializing_none]
#[derive(Builder, Clone, Debug, Default, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
#[builder(setter(into, strip_option), default)]
pub struct ServiceCheck {
    #[serde(rename = "ID")]
    pub id: Option<String>,
    pub namespace: Option<String>,
}

#[skip_serializing_none]
#[derive(Builder, Clone, Debug, Default, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
#[builder(setter(into, strip_option), default)]
pub struct SessionEntry {
    pub behavior: Option<String>,
    pub create_index: Option<u64>,
    #[serde(rename = "ID")]
    pub id: Option<String>,
    pub lock_delay: Option<u64>,
    pub name: Option<String>,
    pub namespace: Option<String>,
    pub node: Option<String>,
    pub node_checks: Option<Vec<String>>,
    pub service_checks: Option<Vec<ServiceCheck>>,
    #[serde(rename = "TTL")]
    pub ttl: Option<String>,
}

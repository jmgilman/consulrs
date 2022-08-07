use derive_builder::Builder;
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::Debug;

use crate::api::{
    check::common::HealthCheck, service::common::AgentService, catalog::common::Node,
};

#[skip_serializing_none]
#[derive(Builder, Clone, Debug, Default, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
#[builder(setter(into, strip_option), default)]
pub struct HealthServiceChecksInfo {
    pub node: Node,
    pub service: AgentService,
    pub checks: Vec<HealthCheck>,
}

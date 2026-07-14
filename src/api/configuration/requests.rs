use crate::api::Features;

use super::common::AgentConfiguration;
use consulrs_derive::QueryEndpoint;
use derive_builder::Builder;
use rustify_derive::Endpoint;
use serde::Serialize;
use std::fmt::Debug;

/// ## Read configuration
/// This endpoint returns the configuration and member information of the local agent.
///
/// * Path: agent/self
/// * Method: GET
/// * Response: [AgentConfiguration]
/// * Reference: https://developer.hashicorp.com/consul/api-docs/agent#read-configuration
#[derive(Builder, Debug, Default, Endpoint, QueryEndpoint)]
#[endpoint(path = "agent/self", response = "AgentConfiguration", builder = "true")]
#[builder(setter(into, strip_option), default)]
pub struct ReadConfigurationRequest {
    #[endpoint(skip)]
    pub features: Option<Features>,
    #[endpoint(query)]
    pub ns: Option<String>,
}

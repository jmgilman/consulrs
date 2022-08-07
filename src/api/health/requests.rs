use super::common::HealthServiceChecksInfo;
use crate::api::Features;
use consulrs_derive::QueryEndpoint;
use derive_builder::Builder;
use rustify_derive::Endpoint;
use serde::Serialize;
use std::fmt::Debug;

/// ## List Service Instances for Service
/// Returns the service instances providing the service indicated.
///
/// * Path: health/service/{self.service}
/// * Method: GET
/// * Response: [Vec<HealthServiceChecksInfo>]
/// * Reference: https://www.consul.io/api-docs/health#list-nodes-for-service
#[derive(Builder, Debug, Default, Endpoint, QueryEndpoint)]
#[endpoint(
    path = "health/service/{self.service}",
    response = "Vec<HealthServiceChecksInfo>",
    builder = "true"
)]
#[builder(setter(into, strip_option), default)]
pub struct ListNodesForServiceRequest {
    #[endpoint(skip)]
    pub features: Option<Features>,
    #[endpoint(skip)]
    pub service: String,
    #[endpoint(query)]
    pub dc: Option<String>,
    #[endpoint(query)]
    pub near: Option<String>,
    #[endpoint(query)]
    pub passing: Option<bool>,
    #[endpoint(query)]
    pub filter: Option<String>,
    #[endpoint(query)]
    pub peer: Option<String>,
    #[endpoint(query)]
    pub ns: Option<String>,
}

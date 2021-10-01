use super::common::{
    AgentService, AgentServiceChecksInfo, AgentServiceConnect, AgentServiceConnectProxy,
    AgentWeights,
};
use crate::api::{check::common::AgentServiceCheck, Features};
use consulrs_derive::QueryEndpoint;
use derive_builder::Builder;
use rustify_derive::Endpoint;
use serde::Serialize;
use std::{collections::HashMap, fmt::Debug};

/// ## List Services
/// This endpoint returns all the services that are registered with the local
/// agent.
///
/// * Path: agent/services
/// * Method: GET
/// * Response: [HashMap<String, AgentService>]
/// * Reference: https://www.consul.io/api-docs/agent/service#list-services
#[derive(Builder, Debug, Default, Endpoint, QueryEndpoint)]
#[endpoint(
    path = "agent/services",
    response = "HashMap<String, AgentService>",
    builder = "true"
)]
#[builder(setter(into, strip_option), default)]
pub struct ListServicesRequest {
    #[endpoint(skip)]
    pub features: Option<Features>,
    #[endpoint(query)]
    pub ns: Option<String>,
}

/// ## Get Service Configuration
/// This endpoint returns the full service definition for a single service
/// instance registered on the local agent.
///
/// * Path: agent/service/{self.name}
/// * Method: GET
/// * Response: [AgentService]
/// * Reference: https://www.consul.io/api-docs/agent/service#get-service-configuration
#[derive(Builder, Debug, Default, Endpoint, QueryEndpoint)]
#[endpoint(
    path = "agent/service/{self.name}",
    response = "AgentService",
    builder = "true"
)]
#[builder(setter(into, strip_option), default)]
pub struct ReadServiceRequest {
    #[endpoint(skip)]
    pub features: Option<Features>,
    #[endpoint(skip)]
    pub name: String,
    #[endpoint(query)]
    pub ns: Option<String>,
}

/// ## Get local service health
/// Retrieve an aggregated state of service(s) on the local agent by name.
///
/// * Path: agent/health/service/name/{self.service}
/// * Method: GET
/// * Response: [Vec<AgentServiceChecksInfo>]
/// * Reference: https://www.consul.io/api-docs/agent/service#get-local-service-health
#[derive(Builder, Debug, Default, Endpoint, QueryEndpoint)]
#[endpoint(
    path = "agent/health/service/name/{self.name}",
    response = "Vec<AgentServiceChecksInfo>",
    builder = "true"
)]
#[builder(setter(into, strip_option), default)]
pub struct ServiceHealthRequest {
    #[endpoint(skip)]
    pub features: Option<Features>,
    #[endpoint(skip)]
    pub name: String,
    #[endpoint(query)]
    pub ns: Option<String>,
}

/// ## Get local service health by ID
/// Retrieve the health state of a specific service on the local agent by ID.
///
/// * Path: agent/health/service/id/{self.id}
/// * Method: GET
/// * Response: [Vec<AgentServiceChecksInfo>]
/// * Reference: https://www.consul.io/api-docs/agent/service#get-local-service-health-by-its-id
#[derive(Builder, Debug, Default, Endpoint, QueryEndpoint)]
#[endpoint(
    path = "agent/health/service/id/{self.id}",
    response = "Vec<AgentServiceChecksInfo>",
    builder = "true"
)]
#[builder(setter(into, strip_option), default)]
pub struct ServiceHealthByIdRequest {
    #[endpoint(skip)]
    pub features: Option<Features>,
    #[endpoint(skip)]
    pub id: String,
    #[endpoint(query)]
    pub ns: Option<String>,
}

/// ## Register Service
/// This endpoint adds a new service, with optional health checks, to the local
/// agent.
///
/// * Path: agent/service/register
/// * Method: PUT
/// * Response: N/A
/// * Reference: https://www.consul.io/api-docs/agent/service#register-service
#[derive(Builder, Clone, Debug, Default, Endpoint, QueryEndpoint, Serialize)]
#[endpoint(path = "agent/service/register", method = "PUT", builder = "true")]
#[serde(rename_all = "PascalCase")]
#[builder(setter(into, strip_option), default)]
pub struct RegisterServiceRequest {
    #[endpoint(skip)]
    #[serde(skip)]
    pub features: Option<Features>,
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

/// ## Deregister Service
/// This endpoint removes a service from the local agent.
///
/// * Path: agent/service/deregister/{self.id}
/// * Method: PUT
/// * Response: N/A
/// * Reference: https://www.consul.io/api-docs/agent/service#deregister-service
#[derive(Builder, Debug, Default, Endpoint, QueryEndpoint)]
#[endpoint(
    path = "agent/service/deregister/{self.id}",
    method = "PUT",
    builder = "true"
)]
#[builder(setter(into, strip_option), default)]
pub struct DeregisterServiceRequest {
    #[endpoint(skip)]
    pub features: Option<Features>,
    #[endpoint(skip)]
    pub id: String,
    #[endpoint(query)]
    pub ns: Option<String>,
}

/// ## Enable Maintenance Mode
/// This endpoint places a given service into "maintenance mode".
///
/// * Path: agent/service/maintenance/{self.id}
/// * Method: PUT
/// * Response: N/A
/// * Reference: https://www.consul.io/api-docs/agent/service#enable-maintenance-mode
#[derive(Builder, Debug, Default, Endpoint, QueryEndpoint)]
#[endpoint(
    path = "agent/service/maintenance/{self.id}",
    method = "PUT",
    builder = "true"
)]
#[builder(setter(into, strip_option), default)]
pub struct EnableMaintenanceRequest {
    #[endpoint(skip)]
    pub features: Option<Features>,
    #[endpoint(skip)]
    pub id: String,
    #[endpoint(query)]
    pub enable: bool,
    #[endpoint(query)]
    pub ns: Option<String>,
}

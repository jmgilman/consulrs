use super::{
    common::{CatalogService, Node},
    responses::{GatewayServiceResponse, ListNodeServicesResponse, ListNodesForServiceResponse},
};
use crate::api::{check::common::AgentCheck, service::common::AgentService, Features};
use consulrs_derive::QueryEndpoint;
use derive_builder::Builder;
use rustify_derive::Endpoint;
use serde::Serialize;
use std::{collections::HashMap, fmt::Debug};

/// ## Register Entity
/// This endpoint is a low-level mechanism for registering or updating entries
/// in the catalog.
///
/// * Path: catalog/register
/// * Method: PUT
/// * Response: [bool]
/// * Reference: https://www.consul.io/api-docs/catalog#register-entity
#[derive(Builder, Debug, Default, Endpoint, QueryEndpoint, Serialize)]
#[endpoint(
    path = "catalog/register",
    method = "PUT",
    response = "bool",
    builder = "true"
)]
#[builder(setter(into, strip_option), default)]
#[serde(rename_all = "PascalCase")]
pub struct RegisterEntityRequest {
    #[serde(skip)]
    #[endpoint(skip)]
    pub features: Option<Features>,
    pub node: String,
    pub address: String,
    pub check: Option<AgentCheck>,
    pub checks: Option<Vec<AgentCheck>>,
    pub datacenter: Option<String>,
    pub tagged_addresses: Option<HashMap<String, String>>,
    pub node_meta: Option<HashMap<String, String>>,
    pub ns: Option<String>,
    pub service: Option<AgentService>,
    pub skip_node_update: Option<bool>,
}

/// ## Deregister Entity
/// This endpoint is a low-level mechanism for directly removing entries from
/// the Catalog.
///
/// * Path: catalog/deregister
/// * Method: PUT
/// * Response: [bool]
/// * Reference: https://www.consul.io/api-docs/catalog#deregister-entity
#[derive(Builder, Debug, Default, Endpoint, QueryEndpoint, Serialize)]
#[endpoint(
    path = "catalog/deregister",
    method = "PUT",
    response = "bool",
    builder = "true"
)]
#[builder(setter(into, strip_option), default)]
#[serde(rename_all = "PascalCase")]
pub struct DeregisterEntityRequest {
    #[serde(skip)]
    #[endpoint(skip)]
    pub features: Option<Features>,
    pub node: String,
    #[serde(rename = "CheckID")]
    pub check_id: Option<String>,
    pub datacenter: Option<String>,
    pub namespace: Option<String>,
    #[serde(rename = "ServiceID")]
    pub service_id: Option<String>,
}

/// ## List Datacenters
/// This endpoint returns the list of all known datacenters.
///
/// * Path: catalog/datacenters
/// * Method: GET
/// * Response: [Vec<String>]
/// * Reference: https://www.consul.io/api-docs/catalog#list-datacenters
#[derive(Builder, Debug, Default, Endpoint, QueryEndpoint)]
#[endpoint(
    path = "catalog/datacenters",
    response = "Vec<String>",
    builder = "true"
)]
#[builder(setter(into, strip_option), default)]
pub struct ListDatacentersRequest {
    #[endpoint(skip)]
    pub features: Option<Features>,
}

/// ## List Nodes
/// This endpoint and returns the nodes registered in a given datacenter.
///
/// * Path: catalog/nodes
/// * Method: GET
/// * Response: [Vec<Node>]
/// * Reference: https://www.consul.io/api-docs/catalog#list-nodes
#[derive(Builder, Debug, Default, Endpoint, QueryEndpoint)]
#[endpoint(path = "catalog/nodes", response = "Vec<Node>", builder = "true")]
#[builder(setter(into, strip_option), default)]
pub struct ListNodesRequest {
    #[endpoint(skip)]
    pub features: Option<Features>,
    #[endpoint(query)]
    pub dc: Option<String>,
    #[endpoint(query)]
    pub near: Option<String>,
}

/// ## List Services
/// This endpoint returns the services registered in a given datacenter.
///
/// * Path: catalog/services
/// * Method: GET
/// * Response: [HashMap<String, Vec<String>>]
/// * Reference: https://www.consul.io/api-docs/catalog#list-services
#[derive(Builder, Debug, Default, Endpoint, QueryEndpoint)]
#[endpoint(
    path = "catalog/services",
    response = "HashMap<String, Vec<String>>",
    builder = "true"
)]
#[builder(setter(into, strip_option), default)]
pub struct ListServicesRequest {
    #[endpoint(skip)]
    pub features: Option<Features>,
    #[endpoint(query)]
    pub dc: Option<String>,
    #[endpoint(query)]
    pub ns: Option<String>,
}

/// ## List Nodes for Service
/// This endpoint returns the nodes providing a service in a given datacenter.
///
/// * Path: catalog/service/{self.service}
/// * Method: GET
/// * Response: [Vec<CatalogService>]
/// * Reference: https://www.consul.io/api-docs/catalog#list-nodes-for-service
#[derive(Builder, Debug, Default, Endpoint, QueryEndpoint)]
#[endpoint(
    path = "catalog/service/{self.service}",
    response = "Vec<CatalogService>",
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
    pub ns: Option<String>,
}

/// ## List Nodes for Connect-capable Service
/// This endpoint returns the nodes providing a Connect-capable service in a
/// given datacenter.
///
/// * Path: catalog/connect/{self.service}
/// * Method: GET
/// * Response: [Vec<ListNodesForServiceResponse>]
/// * Reference: https://www.consul.io/api-docs/catalog#list-nodes-for-connect-capable-service
#[derive(Builder, Debug, Default, Endpoint, QueryEndpoint)]
#[endpoint(
    path = "catalog/connect/{self.service}",
    response = "Vec<ListNodesForServiceResponse>",
    builder = "true"
)]
#[builder(setter(into, strip_option), default)]
pub struct ListNodesForConnectServiceRequest {
    #[endpoint(skip)]
    pub features: Option<Features>,
    #[endpoint(skip)]
    pub service: String,
    #[endpoint(query)]
    pub dc: Option<String>,
    #[endpoint(query)]
    pub ns: Option<String>,
}

/// ## List Services for Node
/// This endpoint returns the node's registered services.
///
/// * Path: catalog/node-services/{self.node}
/// * Method: GET
/// * Response: [ListNodeServicesResponse]
/// * Reference: https://www.consul.io/api-docs/catalog#list-services-for-node
#[derive(Builder, Debug, Default, Endpoint, QueryEndpoint)]
#[endpoint(
    path = "catalog/node-services/{self.node}",
    response = "ListNodeServicesResponse",
    builder = "true"
)]
#[builder(setter(into, strip_option), default)]
pub struct ListNodeServicesRequest {
    #[endpoint(skip)]
    pub features: Option<Features>,
    #[endpoint(skip)]
    pub node: String,
    #[endpoint(query)]
    pub dc: Option<String>,
    #[endpoint(query)]
    pub ns: Option<String>,
}

/// ## List Services for Gateway
/// This endpoint returns the services associated with an ingress gateway or
/// terminating gateway.
///
/// * Path: catalog/gateway-services/{self.gateway}
/// * Method: GET
/// * Response: [Option<Vec<GatewayServiceResponse>>]
/// * Reference: https://www.consul.io/api-docs/catalog#list-services-for-gateway
#[derive(Builder, Debug, Default, Endpoint, QueryEndpoint)]
#[endpoint(
    path = "catalog/gateway-services/{self.gateway}",
    response = "Option<Vec<GatewayServiceResponse>>",
    builder = "true"
)]
#[builder(setter(into, strip_option), default)]
pub struct ListGatewayServicesRequest {
    #[endpoint(skip)]
    pub features: Option<Features>,
    #[endpoint(skip)]
    pub gateway: String,
    #[endpoint(query)]
    pub dc: Option<String>,
    #[endpoint(query)]
    pub ns: Option<String>,
}

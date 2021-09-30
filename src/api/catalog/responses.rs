use serde::Deserialize;
use std::collections::HashMap;

use crate::api::{
    connect::responses::ProxyResponse,
    service::responses::{ConnectResponse, ServiceResponse, WeightsResponse},
};

/// Response from executing
/// [ListNodesRequest][crate::api::catalog::requests::ListNodesRequest]
#[derive(Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct NodeResponse {
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

/// Response from executing
/// [ListNodesForServiceRequest][crate::api::catalog::requests::ListNodesForServiceRequest]
#[derive(Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct ListNodesForServiceResponse {
    pub address: String,
    pub create_index: u64,
    pub datacenter: String,
    #[serde(rename = "ID")]
    pub id: String,
    pub node_meta: Option<HashMap<String, String>>,
    pub modify_index: u64,
    pub namespace: Option<String>,
    pub node: String,
    pub service_address: String,
    pub service_connect: Option<ConnectResponse>,
    pub service_enable_tag_override: bool,
    #[serde(rename = "ServiceID")]
    pub service_id: String,
    pub service_kind: String,
    pub service_meta: Option<HashMap<String, String>>,
    pub service_name: String,
    pub service_port: u64,
    pub service_proxy: ProxyResponse,
    pub service_socket_path: Option<String>,
    pub service_tags: Vec<String>,
    pub service_weights: WeightsResponse,
    pub tagged_addresses: Option<HashMap<String, String>>,
}

/// Response from executing
/// [ListNodeServicesRequest][crate::api::catalog::requests::ListNodeServicesRequest]
#[derive(Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct ListNodeServicesResponse {
    pub node: NodeResponse,
    pub services: Vec<ServiceResponse>,
}

/// Response from executing
/// [ListNodeServicesRequest][crate::api::catalog::requests::ListNodeServicesRequest]
#[derive(Deserialize, Debug)]
pub struct GatewayServiceResponse {
    #[serde(rename = "CAFile")]
    pub ca_file: Option<String>,
    pub cert_file: Option<String>,
    pub from_wildcard: Option<bool>,
    pub gateway: CompoundServiceNameResponse,
    pub gateway_kind: String,
    pub hosts: Option<Vec<String>>,
    pub key_file: Option<String>,
    pub port: Option<u64>,
    pub protocol: Option<String>,
    pub service: CompoundServiceNameResponse,
    #[serde(rename = "SNI")]
    pub sni: Option<String>,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct CompoundServiceNameResponse {
    pub name: String,
    pub namespace: Option<String>,
}

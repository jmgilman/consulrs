use std::collections::HashMap;

use crate::{
    api::{
        self,
        catalog::{
            requests::{
                DeregisterEntityRequest, DeregisterEntityRequestBuilder, ListDatacentersRequest,
                ListDatacentersRequestBuilder, ListGatewayServicesRequest,
                ListGatewayServicesRequestBuilder, ListNodeServicesRequest,
                ListNodeServicesRequestBuilder, ListNodesForConnectServiceRequest,
                ListNodesForConnectServiceRequestBuilder, ListNodesForServiceRequest,
                ListNodesForServiceRequestBuilder, ListNodesRequest, ListNodesRequestBuilder,
                ListServicesRequest, ListServicesRequestBuilder, RegisterEntityRequest,
                RegisterEntityRequestBuilder,
            },
            responses::{
                GatewayServiceResponse, ListNodeServicesResponse, ListNodesForServiceResponse,
                NodeResponse,
            },
        },
        ApiResponse,
    },
    client::Client,
    error::ClientError,
};

/// Lists all known datacenters.
///
/// See [ListDatacentersRequest]
#[instrument(skip(client, opts), err)]
pub async fn datacenters(
    client: &impl Client,
    opts: Option<&mut ListDatacentersRequestBuilder>,
) -> Result<ApiResponse<Vec<String>>, ClientError> {
    let mut t = ListDatacentersRequest::builder();
    let endpoint = opts.unwrap_or(&mut t).build().unwrap();
    api::exec_with_result(client, endpoint).await
}

/// Deregisters an entity.
///
/// See [DeregisterEntityRequest]
#[instrument(skip(client, opts), err)]
pub async fn deregister(
    client: &impl Client,
    node: &str,
    opts: Option<&mut DeregisterEntityRequestBuilder>,
) -> Result<ApiResponse<bool>, ClientError> {
    let mut t = DeregisterEntityRequest::builder();
    let endpoint = opts.unwrap_or(&mut t).node(node).build().unwrap();
    api::exec_with_result(client, endpoint).await
}

/// Lists all services on a gateway.
///
/// See [ListGatewayServicesRequest]
#[instrument(skip(client, opts), err)]
pub async fn gateway(
    client: &impl Client,
    gateway: &str,
    opts: Option<&mut ListGatewayServicesRequestBuilder>,
) -> Result<ApiResponse<Option<Vec<GatewayServiceResponse>>>, ClientError> {
    let mut t = ListGatewayServicesRequest::builder();
    let endpoint = opts.unwrap_or(&mut t).gateway(gateway).build().unwrap();
    api::exec_with_result(client, endpoint).await
}

/// Lists all services on a node.
///
/// See [ListNodeServicesRequest]
#[instrument(skip(client, opts), err)]
pub async fn node(
    client: &impl Client,
    node: &str,
    opts: Option<&mut ListNodeServicesRequestBuilder>,
) -> Result<ApiResponse<ListNodeServicesResponse>, ClientError> {
    let mut t = ListNodeServicesRequest::builder();
    let endpoint = opts.unwrap_or(&mut t).node(node).build().unwrap();
    api::exec_with_result(client, endpoint).await
}

/// Lists all known nodes.
///
/// See [ListNodesRequest]
#[instrument(skip(client, opts), err)]
pub async fn nodes(
    client: &impl Client,
    opts: Option<&mut ListNodesRequestBuilder>,
) -> Result<ApiResponse<Vec<NodeResponse>>, ClientError> {
    let mut t = ListNodesRequest::builder();
    let endpoint = opts.unwrap_or(&mut t).build().unwrap();
    api::exec_with_result(client, endpoint).await
}

/// Lists all nodes providing the given service.
///
/// See [ListNodesForServiceRequest]
#[instrument(skip(client, opts), err)]
pub async fn nodes_with_service(
    client: &impl Client,
    service: &str,
    opts: Option<&mut ListNodesForServiceRequestBuilder>,
) -> Result<ApiResponse<Vec<ListNodesForServiceResponse>>, ClientError> {
    let mut t = ListNodesForServiceRequest::builder();
    let endpoint = opts.unwrap_or(&mut t).service(service).build().unwrap();
    api::exec_with_result(client, endpoint).await
}

/// Lists all nodes providing the given Connect-capabale service.
///
/// See [ListNodesForConnectServiceRequest]
#[instrument(skip(client, opts), err)]
pub async fn nodes_with_connect_service(
    client: &impl Client,
    service: &str,
    opts: Option<&mut ListNodesForConnectServiceRequestBuilder>,
) -> Result<ApiResponse<Vec<ListNodesForServiceResponse>>, ClientError> {
    let mut t = ListNodesForConnectServiceRequest::builder();
    let endpoint = opts.unwrap_or(&mut t).service(service).build().unwrap();
    api::exec_with_result(client, endpoint).await
}

/// Registers an entity.
///
/// See [RegisterEntityRequest]
#[instrument(skip(client, opts), err)]
pub async fn register(
    client: &impl Client,
    node: &str,
    address: &str,
    opts: Option<&mut RegisterEntityRequestBuilder>,
) -> Result<ApiResponse<bool>, ClientError> {
    let mut t = RegisterEntityRequest::builder();
    let endpoint = opts
        .unwrap_or(&mut t)
        .node(node)
        .address(address)
        .build()
        .unwrap();
    api::exec_with_result(client, endpoint).await
}

/// Lists all registered services in a datacenter.
///
/// See [ListServicesRequest]
#[instrument(skip(client, opts), err)]
pub async fn services(
    client: &impl Client,
    opts: Option<&mut ListServicesRequestBuilder>,
) -> Result<ApiResponse<HashMap<String, Vec<String>>>, ClientError> {
    let mut t = ListServicesRequest::builder();
    let endpoint = opts.unwrap_or(&mut t).build().unwrap();
    api::exec_with_result(client, endpoint).await
}

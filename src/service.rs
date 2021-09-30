use std::collections::HashMap;

use crate::{
    api::{
        self,
        service::{
            requests::{
                DeregisterServiceRequest, DeregisterServiceRequestBuilder,
                EnableMaintenanceRequest, EnableMaintenanceRequestBuilder, ListServicesRequest,
                ListServicesRequestBuilder, ReadServiceRequest, ReadServiceRequestBuilder,
                RegisterServiceRequest, RegisterServiceRequestBuilder, ServiceHealthByIdRequest,
                ServiceHealthByIdRequestBuilder, ServiceHealthRequest, ServiceHealthRequestBuilder,
            },
            responses::{ServiceCheckResponse, ServiceResponse},
        },
        ApiResponse,
    },
    client::Client,
    error::ClientError,
};

/// Deregisters a service on an agent.
///
/// See [DeregisterServiceRequest]
#[instrument(skip(client, opts), err)]
pub async fn deregister(
    client: &impl Client,
    id: &str,
    opts: Option<&mut DeregisterServiceRequestBuilder>,
) -> Result<ApiResponse<()>, ClientError> {
    let mut t = DeregisterServiceRequest::builder();
    let endpoint = opts.unwrap_or(&mut t).id(id).build().unwrap();
    api::exec_with_empty(client, endpoint).await
}

/// Reads the health of the given service on an agent.
///
/// See [ServiceHealthRequest]
#[instrument(skip(client, opts), err)]
pub async fn health(
    client: &impl Client,
    name: &str,
    opts: Option<&mut ServiceHealthRequestBuilder>,
) -> Result<ApiResponse<Vec<ServiceCheckResponse>>, ClientError> {
    let mut t = ServiceHealthRequest::builder();
    let endpoint = opts.unwrap_or(&mut t).name(name).build().unwrap();
    api::exec_with_result(client, endpoint).await
}

/// Reads the health of the given service on an agent using a service ID.
///
/// See [ServiceHealthByIdRequest]
#[instrument(skip(client, opts), err)]
pub async fn health_by_id(
    client: &impl Client,
    id: &str,
    opts: Option<&mut ServiceHealthByIdRequestBuilder>,
) -> Result<ApiResponse<Vec<ServiceCheckResponse>>, ClientError> {
    let mut t = ServiceHealthByIdRequest::builder();
    let endpoint = opts.unwrap_or(&mut t).id(id).build().unwrap();
    api::exec_with_result(client, endpoint).await
}

/// Lists all registered services on an agent.
///
/// See [ListServicesRequest]
#[instrument(skip(client, opts), err)]
pub async fn list(
    client: &impl Client,
    opts: Option<&mut ListServicesRequestBuilder>,
) -> Result<ApiResponse<HashMap<String, ServiceResponse>>, ClientError> {
    let mut t = ListServicesRequest::builder();
    let endpoint = opts.unwrap_or(&mut t).build().unwrap();
    api::exec_with_result(client, endpoint).await
}

/// Places a service in maintenance mode on an agent.
///
/// See [EnableMaintenanceRequest]
#[instrument(skip(client, opts), err)]
pub async fn maintenance(
    client: &impl Client,
    id: &str,
    enabled: bool,
    opts: Option<&mut EnableMaintenanceRequestBuilder>,
) -> Result<ApiResponse<()>, ClientError> {
    let mut t = EnableMaintenanceRequest::builder();
    let endpoint = opts
        .unwrap_or(&mut t)
        .id(id)
        .enable(enabled)
        .build()
        .unwrap();
    api::exec_with_empty(client, endpoint).await
}

/// Reads the given service's configuration on an agent.
///
/// See [ReadServiceRequest]
#[instrument(skip(client, opts), err)]
pub async fn read(
    client: &impl Client,
    name: &str,
    opts: Option<&mut ReadServiceRequestBuilder>,
) -> Result<ApiResponse<ServiceResponse>, ClientError> {
    let mut t = ReadServiceRequest::builder();
    let endpoint = opts.unwrap_or(&mut t).name(name).build().unwrap();
    api::exec_with_result(client, endpoint).await
}

/// Registers a new service on an agent.
///
/// See [RegisterServiceRequest]
#[instrument(skip(client, opts), err)]
pub async fn register(
    client: &impl Client,
    name: &str,
    opts: Option<&mut RegisterServiceRequestBuilder>,
) -> Result<ApiResponse<()>, ClientError> {
    let mut t = RegisterServiceRequest::builder();
    let endpoint = opts.unwrap_or(&mut t).name(name).build().unwrap();
    api::exec_with_empty(client, endpoint).await
}

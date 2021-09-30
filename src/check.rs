use std::collections::HashMap;

use crate::{
    api::{
        self,
        check::{
            requests::{
                DeregisterCheckRequest, DeregisterCheckRequestBuilder, ListChecksRequest,
                ListChecksRequestBuilder, RegisterCheckRequest, RegisterCheckRequestBuilder,
                TtlCheckFailRequest, TtlCheckFailRequestBuilder, TtlCheckPassRequest,
                TtlCheckPassRequestBuilder, TtlCheckUpdateRequest, TtlCheckUpdateRequestBuilder,
                TtlCheckWarnRequest, TtlCheckWarnRequestBuilder,
            },
            responses::AgentCheckResponse,
        },
        ApiResponse,
    },
    client::Client,
    error::ClientError,
};

/// Deregisters a check on an agent.
///
/// See [DeregisterCheckRequest]
#[instrument(skip(client, opts), err)]
pub async fn deregister(
    client: &impl Client,
    name: &str,
    opts: Option<&mut DeregisterCheckRequestBuilder>,
) -> Result<ApiResponse<()>, ClientError> {
    let mut t = DeregisterCheckRequest::builder();
    let endpoint = opts.unwrap_or(&mut t).check(name).build().unwrap();
    api::exec_with_empty(client, endpoint).await
}

/// Sets the status of a TTL check to critical.
///
/// See [TtlCheckFailRequest]
#[instrument(skip(client, opts), err)]
pub async fn fail(
    client: &impl Client,
    name: &str,
    opts: Option<&mut TtlCheckFailRequestBuilder>,
) -> Result<ApiResponse<()>, ClientError> {
    let mut t = TtlCheckFailRequest::builder();
    let endpoint = opts.unwrap_or(&mut t).check(name).build().unwrap();
    api::exec_with_empty(client, endpoint).await
}

/// Lists all registered checks on an agent.
///
/// See [ListChecksRequest]
#[instrument(skip(client, opts), err)]
pub async fn list(
    client: &impl Client,
    opts: Option<&mut ListChecksRequestBuilder>,
) -> Result<ApiResponse<HashMap<String, AgentCheckResponse>>, ClientError> {
    let mut t = ListChecksRequest::builder();
    let endpoint = opts.unwrap_or(&mut t).build().unwrap();
    api::exec_with_result(client, endpoint).await
}

/// Sets the status of a TTL check to passing.
///
/// See [TtlCheckPassRequest]
#[instrument(skip(client, opts), err)]
pub async fn pass(
    client: &impl Client,
    name: &str,
    opts: Option<&mut TtlCheckPassRequestBuilder>,
) -> Result<ApiResponse<()>, ClientError> {
    let mut t = TtlCheckPassRequest::builder();
    let endpoint = opts.unwrap_or(&mut t).check(name).build().unwrap();
    api::exec_with_empty(client, endpoint).await
}

/// Registers a new check on an agent.
///
/// See [RegisterCheckRequest]
#[instrument(skip(client, opts), err)]
pub async fn register(
    client: &impl Client,
    name: &str,
    opts: Option<&mut RegisterCheckRequestBuilder>,
) -> Result<ApiResponse<()>, ClientError> {
    let mut t = RegisterCheckRequest::builder();
    let endpoint = opts.unwrap_or(&mut t).name(name).build().unwrap();
    api::exec_with_empty(client, endpoint).await
}

/// Sets the status of a TTL check to the specified status.
///
/// See [TtlCheckUpdateRequest]
#[instrument(skip(client, opts), err)]
pub async fn set_status(
    client: &impl Client,
    name: &str,
    status: &str,
    opts: Option<&mut TtlCheckUpdateRequestBuilder>,
) -> Result<ApiResponse<()>, ClientError> {
    let mut t = TtlCheckUpdateRequest::builder();
    let endpoint = opts
        .unwrap_or(&mut t)
        .check(name)
        .status(status)
        .build()
        .unwrap();
    api::exec_with_empty(client, endpoint).await
}

/// Sets the status of a TTL check to warning.
///
/// See [TtlCheckWarnRequest]
#[instrument(skip(client, opts), err)]
pub async fn warn(
    client: &impl Client,
    name: &str,
    opts: Option<&mut TtlCheckWarnRequestBuilder>,
) -> Result<ApiResponse<()>, ClientError> {
    let mut t = TtlCheckWarnRequest::builder();
    let endpoint = opts.unwrap_or(&mut t).check(name).build().unwrap();
    api::exec_with_empty(client, endpoint).await
}

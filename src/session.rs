use crate::{
    api::{
        self,
        session::{
            common::SessionEntry,
            requests::{
                CreateSessionRequest, CreateSessionRequestBuilder, DeleteSessionRequest,
                DeleteSessionRequestBuilder, ListNodeSessionsRequest,
                ListNodeSessionsRequestBuilder, ListSessionsRequest, ListSessionsRequestBuilder,
                ReadSessionRequest, ReadSessionRequestBuilder, RenewSessionRequest,
                RenewSessionRequestBuilder,
            },
            responses::CreateSessionResponse,
        },
        ApiResponse,
    },
    client::Client,
    error::ClientError,
};

/// Creates a new session.
///
/// See [CreateSessionRequest]
#[instrument(skip(client, opts), err)]
pub async fn create(
    client: &impl Client,
    opts: Option<&mut CreateSessionRequestBuilder>,
) -> Result<ApiResponse<CreateSessionResponse>, ClientError> {
    let mut t = CreateSessionRequest::builder();
    let endpoint = opts.unwrap_or(&mut t).build().unwrap();
    api::exec_with_result(client, endpoint).await
}

/// Deletes a session.
///
/// See [DeleteSessionRequest]
#[instrument(skip(client, opts), err)]
pub async fn delete(
    client: &impl Client,
    uuid: &str,
    opts: Option<&mut DeleteSessionRequestBuilder>,
) -> Result<ApiResponse<()>, ClientError> {
    let mut t = DeleteSessionRequest::builder();
    let endpoint = opts.unwrap_or(&mut t).uuid(uuid).build().unwrap();
    api::exec_with_empty(client, endpoint).await
}

/// Lists all active sessions.
///
/// See [ListSessionsRequest]
#[instrument(skip(client, opts), err)]
pub async fn list(
    client: &impl Client,
    opts: Option<&mut ListSessionsRequestBuilder>,
) -> Result<ApiResponse<Vec<SessionEntry>>, ClientError> {
    let mut t = ListSessionsRequest::builder();
    let endpoint = opts.unwrap_or(&mut t).build().unwrap();
    api::exec_with_result(client, endpoint).await
}

/// Lists all active sessions on the given node.
///
/// See [ListNodeSessionsRequest]
#[instrument(skip(client, opts), err)]
pub async fn list_by_node(
    client: &impl Client,
    node: &str,
    opts: Option<&mut ListNodeSessionsRequestBuilder>,
) -> Result<ApiResponse<Vec<SessionEntry>>, ClientError> {
    let mut t = ListNodeSessionsRequest::builder();
    let endpoint = opts.unwrap_or(&mut t).node(node).build().unwrap();
    api::exec_with_result(client, endpoint).await
}

/// Reads the information about a session.
///
/// See [ReadSessionRequest]
#[instrument(skip(client, opts), err)]
pub async fn read(
    client: &impl Client,
    uuid: &str,
    opts: Option<&mut ReadSessionRequestBuilder>,
) -> Result<ApiResponse<Vec<SessionEntry>>, ClientError> {
    let mut t = ReadSessionRequest::builder();
    let endpoint = opts.unwrap_or(&mut t).uuid(uuid).build().unwrap();
    api::exec_with_result(client, endpoint).await
}

/// Renews the session.
///
/// See [RenewSessionRequest]
#[instrument(skip(client, opts), err)]
pub async fn renew(
    client: &impl Client,
    uuid: &str,
    opts: Option<&mut RenewSessionRequestBuilder>,
) -> Result<ApiResponse<Vec<SessionEntry>>, ClientError> {
    let mut t = RenewSessionRequest::builder();
    let endpoint = opts.unwrap_or(&mut t).uuid(uuid).build().unwrap();
    api::exec_with_result(client, endpoint).await
}

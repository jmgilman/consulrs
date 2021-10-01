use crate::{
    api::{
        self,
        snapshot::requests::{
            GenerateSnapshotRequest, GenerateSnapshotRequestBuilder, RestoreSnapshotRequest,
            RestoreSnapshotRequestBuilder,
        },
        ApiResponse,
    },
    client::Client,
    error::ClientError,
};

/// Takes a point-in-time snapshot of the Consul cluster.
///
/// See [GenerateSnapshotRequest]
#[instrument(skip(client, opts), err)]
pub async fn backup(
    client: &impl Client,
    opts: Option<&mut GenerateSnapshotRequestBuilder>,
) -> Result<ApiResponse<Vec<u8>>, ClientError> {
    let mut t = GenerateSnapshotRequest::builder();
    let endpoint = opts.unwrap_or(&mut t).build().unwrap();
    api::exec_with_raw(client, endpoint).await
}

/// Restores a snapshot to the Consul cluster.
///
/// See [RestoreSnapshotRequest]
#[instrument(skip(client, opts), err)]
pub async fn restore(
    client: &impl Client,
    data: &[u8],
    opts: Option<&mut RestoreSnapshotRequestBuilder>,
) -> Result<ApiResponse<()>, ClientError> {
    let mut t = RestoreSnapshotRequest::builder();
    let endpoint = opts.unwrap_or(&mut t).data(data).build().unwrap();
    api::exec_with_empty(client, endpoint).await
}

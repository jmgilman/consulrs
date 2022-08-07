use crate::{
    api::{
        self,
        health::{
            common::HealthServiceChecksInfo,
            requests::{
                ListNodesForServiceRequest, ListNodesForServiceRequestBuilder,
            },
        },
        ApiResponse,
    },
    client::Client,
    error::ClientError,
};

/// List Service Instances for Service.
///
/// See [ListNodesForServiceRequest]
#[instrument(skip(client, opts), err)]
pub async fn list_nodes_for_service(
    client: &impl Client,
    service: &str,
    opts: Option<&mut ListNodesForServiceRequestBuilder>,
) -> Result<ApiResponse<Vec<HealthServiceChecksInfo>>, ClientError> {
    let mut t = ListNodesForServiceRequest::builder();
    let endpoint = opts.unwrap_or(&mut t).service(service).build().unwrap();
    api::exec_with_result(client, endpoint).await
}

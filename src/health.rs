use crate::{
    api::{
        self,
        health::{
            common::HealthServiceChecksInfo,
            requests::{
                ListServiceHealthRequest, ListServiceHealthRequestBuilder,
            },
        },
        ApiResponse,
    },
    client::Client,
    error::ClientError,
};

/// List Service Instances for Service.
///
/// See [ListServiceHealthRequest]
#[instrument(skip(client, opts), err)]
pub async fn list_service_instances(
    client: &impl Client,
    service: &str,
    opts: Option<&mut ListServiceHealthRequestBuilder>,
) -> Result<ApiResponse<Vec<HealthServiceChecksInfo>>, ClientError> {
    let mut t = ListServiceHealthRequest::builder();
    let endpoint = opts.unwrap_or(&mut t).service(service).build().unwrap();
    api::exec_with_result(client, endpoint).await
}

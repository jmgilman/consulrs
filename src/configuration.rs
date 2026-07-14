use crate::{
    api::{
        self,
        configuration::{
            common::AgentConfiguration,
            requests::{ReadConfigurationRequest, ReadConfigurationRequestBuilder},
        },
        ApiResponse,
    },
    client::Client,
    error::ClientError,
};

/// Reads the configuration and member information of the local agent.
///
/// See [ReadConfigurationRequest]
#[instrument(skip(client, opts), err)]
pub async fn read(
    client: &impl Client,
    opts: Option<&mut ReadConfigurationRequestBuilder>,
) -> Result<ApiResponse<AgentConfiguration>, ClientError> {
    let mut t = ReadConfigurationRequest::builder();
    let endpoint = opts.unwrap_or(&mut t).build().unwrap();
    api::exec_with_result(client, endpoint).await
}

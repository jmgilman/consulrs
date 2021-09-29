use crate::{
    api::{
        self,
        kv::{
            requests::{
                DeleteKeyRequest, DeleteKeyRequestBuilder, ReadKeyRequest, ReadKeyRequestBuilder,
                ReadKeysRequest, ReadKeysRequestBuilder, ReadRawKeyRequest,
                ReadRawKeyRequestBuilder, SetKeyRequest, SetKeyRequestBuilder,
            },
            responses::ReadKeyResponse,
        },
        ApiResponse,
    },
    client::Client,
    error::ClientError,
};

/// Deletes the given key.
///
/// See [DeleteKeyRequest]
#[instrument(skip(client, opts), err)]
pub async fn delete(
    client: &impl Client,
    key: &str,
    opts: Option<&mut DeleteKeyRequestBuilder>,
) -> Result<ApiResponse<bool>, ClientError> {
    let mut t = DeleteKeyRequest::builder();
    let endpoint = opts.unwrap_or(&mut t).key(key).build().unwrap();
    api::exec_with_result(client, endpoint).await
}

/// Reads the value at the given key.
///
/// See [ReadKeysRequest]
#[instrument(skip(client, opts), err)]
pub async fn keys(
    client: &impl Client,
    path: &str,
    opts: Option<&mut ReadKeysRequestBuilder>,
) -> Result<ApiResponse<Vec<String>>, ClientError> {
    let mut t = ReadKeysRequest::builder();
    let endpoint = opts.unwrap_or(&mut t).key(path).build().unwrap();
    api::exec_with_result(client, endpoint).await
}

/// Reads the raw value at the given key.
///
/// See [ReadKeyRequest]
#[instrument(skip(client, opts), err)]
pub async fn raw(
    client: &impl Client,
    key: &str,
    opts: Option<&mut ReadRawKeyRequestBuilder>,
) -> Result<ApiResponse<Vec<u8>>, ClientError> {
    let mut t = ReadRawKeyRequest::builder();
    let endpoint = opts.unwrap_or(&mut t).key(key).build().unwrap();
    api::exec_with_raw(client, endpoint).await
}

/// Reads the value at the given key.
///
/// See [ReadKeyRequest]
#[instrument(skip(client, opts), err)]
pub async fn read(
    client: &impl Client,
    key: &str,
    opts: Option<&mut ReadKeyRequestBuilder>,
) -> Result<ApiResponse<Vec<ReadKeyResponse>>, ClientError> {
    let mut t = ReadKeyRequest::builder();
    let endpoint = opts.unwrap_or(&mut t).key(key).build().unwrap();
    api::exec_with_result(client, endpoint).await
}

/// Sets the value at the given key.
///
/// See [SetKeyRequest]
#[instrument(skip(client, value, opts), err)]
pub async fn set<'a>(
    client: &'a impl Client,
    key: &'a str,
    value: &'static [u8],
    opts: Option<&'a mut SetKeyRequestBuilder>,
) -> Result<ApiResponse<bool>, ClientError> {
    let mut t = SetKeyRequest::builder();
    let endpoint = opts
        .unwrap_or(&mut t)
        .key(key)
        .value(value)
        .build()
        .unwrap();
    api::exec_with_result(client, endpoint).await
}

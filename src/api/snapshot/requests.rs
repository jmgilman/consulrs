use crate::api::Features;
use consulrs_derive::QueryEndpoint;
use derive_builder::Builder;
use rustify_derive::Endpoint;
use serde::Serialize;
use std::fmt::Debug;

/// ## Generate Snapshot
/// This endpoint generates and returns an atomic, point-in-time snapshot of the
/// Consul server state.
///
/// * Path: snapshot
/// * Method: GET
/// * Response: N/A
/// * Reference: https://www.consul.io/api-docs/snapshot#generate-snapshot
#[derive(Builder, Clone, Debug, Default, Endpoint, QueryEndpoint, Serialize)]
#[endpoint(path = "snapshot", builder = "true")]
#[serde(rename_all = "PascalCase")]
#[builder(setter(into, strip_option), default)]
pub struct GenerateSnapshotRequest {
    #[endpoint(skip)]
    #[serde(skip)]
    pub features: Option<Features>,
    #[endpoint(query)]
    pub dc: Option<String>,
    #[endpoint(query)]
    pub stale: Option<bool>,
}

/// ## Restore Snapshot
/// This endpoint restores a point-in-time snapshot of the Consul server state.
///
/// * Path: snapshot
/// * Method: PUT
/// * Response: N/A
/// * Reference: https://www.consul.io/api-docs/snapshot#restore-snapshot
#[derive(Builder, Clone, Debug, Default, Endpoint, QueryEndpoint, Serialize)]
#[endpoint(path = "snapshot", method = "PUT", builder = "true")]
#[serde(rename_all = "PascalCase")]
#[builder(setter(into, strip_option), default)]
pub struct RestoreSnapshotRequest {
    #[endpoint(skip)]
    #[serde(skip)]
    pub features: Option<Features>,
    #[endpoint(raw)]
    pub data: Vec<u8>,
    #[endpoint(query)]
    pub dc: Option<String>,
}

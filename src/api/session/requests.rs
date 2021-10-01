use super::{
    common::{ServiceCheck, SessionEntry},
    responses::CreateSessionResponse,
};
use crate::api::Features;
use consulrs_derive::QueryEndpoint;
use derive_builder::Builder;
use rustify_derive::Endpoint;
use serde::Serialize;
use std::fmt::Debug;

/// ## Create Session
/// This endpoint initializes a new session.
///
/// * Path: session/create
/// * Method: PUT
/// * Response: [CreateSessionResponse]
/// * Reference: https://www.consul.io/api-docs/session#create-session
#[derive(Builder, Clone, Debug, Default, Endpoint, QueryEndpoint, Serialize)]
#[endpoint(
    path = "session/create",
    method = "PUT",
    response = "CreateSessionResponse",
    builder = "true"
)]
#[serde(rename_all = "PascalCase")]
#[builder(setter(into, strip_option), default)]
pub struct CreateSessionRequest {
    #[endpoint(skip)]
    #[serde(skip)]
    pub features: Option<Features>,
    pub behavior: Option<String>,
    pub create_index: Option<u64>,
    #[serde(rename = "ID")]
    pub id: Option<String>,
    pub lock_delay: Option<String>,
    pub name: Option<String>,
    pub namespace: Option<String>,
    pub node: Option<String>,
    pub node_checks: Option<Vec<String>>,
    pub service_checks: Option<Vec<ServiceCheck>>,
    #[serde(rename = "TTL")]
    pub ttl: Option<String>,
}

/// ## Delete Session
/// This endpoint destroys the session with the given name.
///
/// * Path: session/destroy/{self.uuid}
/// * Method: PUT
/// * Response: N/A
/// * Reference: https://www.consul.io/api-docs/session#delete-session
#[derive(Builder, Clone, Debug, Default, Endpoint, QueryEndpoint, Serialize)]
#[endpoint(path = "session/destroy/{self.uuid}", method = "PUT", builder = "true")]
#[serde(rename_all = "PascalCase")]
#[builder(setter(into, strip_option), default)]
pub struct DeleteSessionRequest {
    #[endpoint(skip)]
    #[serde(skip)]
    pub features: Option<Features>,
    #[endpoint(skip)]
    pub uuid: String,
    #[endpoint(query)]
    pub dc: Option<String>,
    #[endpoint(query)]
    pub ns: Option<String>,
}

/// ## Read Session
/// This endpoint returns the requested session information.
///
/// * Path: session/info/{self.uuid}
/// * Method: GET
/// * Response: [Vec<SessionEntry>]
/// * Reference: https://www.consul.io/api-docs/session#read-session
#[derive(Builder, Clone, Debug, Default, Endpoint, QueryEndpoint, Serialize)]
#[endpoint(
    path = "session/info/{self.uuid}",
    response = "Vec<SessionEntry>",
    builder = "true"
)]
#[serde(rename_all = "PascalCase")]
#[builder(setter(into, strip_option), default)]
pub struct ReadSessionRequest {
    #[endpoint(skip)]
    #[serde(skip)]
    pub features: Option<Features>,
    #[endpoint(skip)]
    pub uuid: String,
    #[endpoint(query)]
    pub dc: Option<String>,
    #[endpoint(query)]
    pub ns: Option<String>,
}

/// ## List Sessions for Node
/// This endpoint returns the active sessions for a given node.
///
/// * Path: session/node/{self.node}
/// * Method: GET
/// * Response: [Vec<SessionEntry>]
/// * Reference: https://www.consul.io/api-docs/session#list-sessions-for-node
#[derive(Builder, Clone, Debug, Default, Endpoint, QueryEndpoint, Serialize)]
#[endpoint(
    path = "session/node/{self.node}",
    response = "Vec<SessionEntry>",
    builder = "true"
)]
#[serde(rename_all = "PascalCase")]
#[builder(setter(into, strip_option), default)]
pub struct ListNodeSessionsRequest {
    #[endpoint(skip)]
    #[serde(skip)]
    pub features: Option<Features>,
    #[endpoint(skip)]
    pub node: String,
    #[endpoint(query)]
    pub dc: Option<String>,
    #[endpoint(query)]
    pub ns: Option<String>,
}

/// ## List Sessions
/// This endpoint returns the list of active sessions.
///
/// * Path: session/list
/// * Method: GET
/// * Response: [Vec<SessionEntry>]
/// * Reference: https://www.consul.io/api-docs/session#list-sessions
#[derive(Builder, Clone, Debug, Default, Endpoint, QueryEndpoint, Serialize)]
#[endpoint(
    path = "session/list",
    response = "Vec<SessionEntry>",
    builder = "true"
)]
#[serde(rename_all = "PascalCase")]
#[builder(setter(into, strip_option), default)]
pub struct ListSessionsRequest {
    #[endpoint(skip)]
    #[serde(skip)]
    pub features: Option<Features>,
    #[endpoint(query)]
    pub dc: Option<String>,
    #[endpoint(query)]
    pub ns: Option<String>,
}

/// ## Renew Session
/// This endpoint renews the given session.
///
/// * Path: session/renew/{self.uuid}
/// * Method: PUT
/// * Response: [Vec<SessionEntry>]
/// * Reference: https://www.consul.io/api-docs/session#renew-session
#[derive(Builder, Clone, Debug, Default, Endpoint, QueryEndpoint, Serialize)]
#[endpoint(
    path = "session/renew/{self.uuid}",
    method = "PUT",
    response = "Vec<SessionEntry>",
    builder = "true"
)]
#[serde(rename_all = "PascalCase")]
#[builder(setter(into, strip_option), default)]
pub struct RenewSessionRequest {
    #[endpoint(skip)]
    #[serde(skip)]
    pub features: Option<Features>,
    #[endpoint(skip)]
    pub uuid: String,
    #[endpoint(query)]
    pub dc: Option<String>,
    #[endpoint(query)]
    pub ns: Option<String>,
}

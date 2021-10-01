use serde::Deserialize;

#[derive(Clone, Debug, Deserialize)]
pub struct CreateSessionResponse {
    #[serde(rename = "ID")]
    pub id: String,
}

use serde::{Deserialize, Serialize};
#[derive(Default, Debug, Serialize, Deserialize)]
/// AuthReport describes the response for authentication check
pub struct AuthReport {
    #[serde(rename = "IdentityToken")]
    pub identity_token: Option<String>,
    #[serde(rename = "Status")]
    pub status: Option<String>,
}

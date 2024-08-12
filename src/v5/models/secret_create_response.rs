use serde::{Deserialize, Serialize};
#[derive(Default, Debug, Serialize, Deserialize)]
pub struct SecretCreateResponse {
    #[serde(rename = "ID")]
    pub id: Option<String>,
}

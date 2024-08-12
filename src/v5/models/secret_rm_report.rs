use serde::{Deserialize, Serialize};
#[derive(Default, Debug, Serialize, Deserialize)]
pub struct SecretRmReport {
    #[serde(rename = "Err")]
    pub err: Option<String>,
    #[serde(rename = "ID")]
    pub id: Option<String>,
}

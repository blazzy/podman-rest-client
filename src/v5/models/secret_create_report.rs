use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Serialize, Deserialize)]
pub struct SecretCreateReport {
    #[serde(rename = "ID")]
    pub id: Option<String>,
}

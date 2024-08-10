use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Serialize, Deserialize)]
/// ServiceUpdateResponse service update response
pub struct ServiceUpdateResponse {
    /// Optional warning messages
    #[serde(rename = "Warnings")]
    pub warnings: Option<Vec<String>>,
}

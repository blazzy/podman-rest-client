use serde::{Deserialize, Serialize};
#[derive(Default, Debug, Serialize, Deserialize)]
pub struct PruneReport {
    #[serde(rename = "Err")]
    pub err: Option<String>,
    #[serde(rename = "Id")]
    pub id: Option<String>,
    #[serde(rename = "Size")]
    pub size: Option<u64>,
}

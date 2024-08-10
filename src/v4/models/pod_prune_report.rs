use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Serialize, Deserialize)]
pub struct PodPruneReport {
    #[serde(rename = "Err")]
    pub err: Option<String>,

    #[serde(rename = "Id")]
    pub id: Option<String>,
}

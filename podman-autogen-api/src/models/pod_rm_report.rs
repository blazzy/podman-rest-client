use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Serialize, Deserialize)]
pub struct PodRmReport {
    #[serde(rename = "Err")]
    pub err: Option<String>,

    #[serde(rename = "Id")]
    pub id: Option<String>,

    #[serde(rename = "RemovedCtrs")]
    pub removed_ctrs: Option<std::collections::HashMap<String, String>>,
}

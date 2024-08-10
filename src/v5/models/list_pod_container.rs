use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Serialize, Deserialize)]
pub struct ListPodContainer {
    #[serde(rename = "Id")]
    pub id: Option<String>,

    #[serde(rename = "Names")]
    pub names: Option<String>,

    #[serde(rename = "RestartCount")]
    pub restart_count: Option<u64>,

    #[serde(rename = "Status")]
    pub status: Option<String>,
}

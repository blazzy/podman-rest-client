use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Serialize, Deserialize)]
pub struct InspectIdMappings {
    #[serde(rename = "GidMap")]
    pub gid_map: Option<Vec<String>>,

    #[serde(rename = "UidMap")]
    pub uid_map: Option<Vec<String>>,
}

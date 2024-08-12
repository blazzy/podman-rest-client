use serde::{Deserialize, Serialize};
#[derive(Default, Debug, Serialize, Deserialize)]
pub struct VolumeRmReport {
    #[serde(rename = "Err")]
    pub err: Option<String>,
    #[serde(rename = "Id")]
    pub id: Option<String>,
}

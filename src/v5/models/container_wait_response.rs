use serde::{Deserialize, Serialize};
#[derive(Default, Debug, Serialize, Deserialize)]
pub struct ContainerWaitResponse {
    #[serde(rename = "Error")]
    pub error: Option<super::super::models::ContainerWaitResponseError>,
    /// container exit code
    #[serde(rename = "StatusCode")]
    pub status_code: Option<i64>,
}

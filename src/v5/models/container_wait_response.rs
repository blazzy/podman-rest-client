use serde::{Deserialize, Serialize};
#[derive(Default, Debug, Serialize, Deserialize)]
pub struct ContainerWaitResponse {
    #[serde(rename = "Error")]
    pub error: Option<crate::v5::models::ContainerWaitResponseError>,
    /// container exit code
    #[serde(rename = "StatusCode")]
    pub status_code: Option<i64>,
}

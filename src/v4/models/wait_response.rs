use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Serialize, Deserialize)]
/// WaitResponse ContainerWaitResponse
/// OK response to ContainerWait operation
pub struct WaitResponse {
    #[serde(rename = "Error")]
    pub error: Option<super::super::models::WaitExitError>,

    /// Exit code of the container
    #[serde(rename = "StatusCode")]
    pub status_code: i64,
}

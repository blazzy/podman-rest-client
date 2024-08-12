use serde::{Deserialize, Serialize};
#[derive(Default, Debug, Serialize, Deserialize)]
pub struct ContainersPruneReportLibpod {
    /// Error which occurred during prune operation (if any).
    /// This field is optional and may be omitted if no error occurred.
    #[serde(rename = "Err")]
    pub err: Option<String>,
    #[serde(rename = "Id")]
    pub id: Option<String>,
    #[serde(rename = "Size")]
    pub size: Option<i64>,
}

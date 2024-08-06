use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Serialize, Deserialize)]
pub struct ManifestPushReport {
    /// ID of the pushed manifest
    #[serde(rename = "Id")]
    pub id: Option<String>,

    /// Error contains text of errors from pushing
    pub error: Option<String>,

    /// Stream used to provide push progress
    pub stream: Option<String>,
}

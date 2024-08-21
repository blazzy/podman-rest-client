use serde::{Deserialize, Serialize};
#[derive(Default, Debug, Serialize, Deserialize)]
/// ListResponse VolumeListResponse
/// Volume list response
pub struct ListResponse {
    /// List of volumes
    #[serde(rename = "Volumes")]
    pub volumes: Option<Vec<crate::v5::models::Volume>>,
    /// Warnings that occurred when fetching the list of volumes.
    #[serde(rename = "Warnings")]
    pub warnings: Option<Vec<String>>,
}

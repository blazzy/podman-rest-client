use serde::{Deserialize, Serialize};
#[derive(Default, Debug, Serialize, Deserialize)]
/// AccessMode defines the access mode of a volume.
pub struct AccessMode {
    #[serde(rename = "BlockVolume")]
    pub block_volume: Option<serde_json::Value>,
    #[serde(rename = "MountVolume")]
    pub mount_volume: Option<super::super::models::TypeMount>,
    #[serde(rename = "Scope")]
    pub scope: Option<String>,
    #[serde(rename = "Sharing")]
    pub sharing: Option<String>,
}

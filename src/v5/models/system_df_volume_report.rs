use serde::{Deserialize, Serialize};
#[derive(Default, Debug, Serialize, Deserialize)]
/// SystemDfVolumeReport describes a volume and its size
pub struct SystemDfVolumeReport {
    #[serde(rename = "Links")]
    pub links: Option<i64>,
    #[serde(rename = "ReclaimableSize")]
    pub reclaimable_size: Option<i64>,
    #[serde(rename = "Size")]
    pub size: Option<i64>,
    #[serde(rename = "VolumeName")]
    pub volume_name: Option<String>,
}

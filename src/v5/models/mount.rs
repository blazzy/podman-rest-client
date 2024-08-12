use serde::{Deserialize, Serialize};
#[derive(Default, Debug, Serialize, Deserialize)]
/// Mount represents a mount (volume).
pub struct Mount {
    #[serde(rename = "BindOptions")]
    pub bind_options: Option<super::super::models::BindOptions>,
    #[serde(rename = "ClusterOptions")]
    pub cluster_options: Option<serde_json::Value>,
    #[serde(rename = "Consistency")]
    pub consistency: Option<String>,
    #[serde(rename = "ReadOnly")]
    pub read_only: Option<bool>,
    /// Source specifies the name of the mount. Depending on mount type, this
    /// may be a volume name or a host path, or even ignored.
    /// Source is not supported for tmpfs (must be an empty value)
    #[serde(rename = "Source")]
    pub source: Option<String>,
    #[serde(rename = "Destination")]
    pub destination: Option<String>,
    #[serde(rename = "TmpfsOptions")]
    pub tmpfs_options: Option<super::super::models::TmpfsOptions>,
    #[serde(rename = "Type")]
    pub r#type: Option<String>,
    #[serde(rename = "VolumeOptions")]
    pub volume_options: Option<super::super::models::VolumeOptions>,
}

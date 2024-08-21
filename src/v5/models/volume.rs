use serde::{Deserialize, Serialize};
#[derive(Default, Debug, Serialize, Deserialize)]
/// Volume volume
pub struct Volume {
    #[serde(rename = "ClusterVolume")]
    pub cluster_volume: Option<crate::v5::models::ClusterVolume>,
    /// Date/Time the volume was created.
    #[serde(rename = "CreatedAt")]
    pub created_at: Option<String>,
    /// Name of the volume driver used by the volume.
    #[serde(rename = "Driver")]
    pub driver: String,
    /// User-defined key/value metadata.
    #[serde(rename = "Labels")]
    pub labels: std::collections::HashMap<String, String>,
    /// Mount path of the volume on the host.
    #[serde(rename = "Mountpoint")]
    pub mountpoint: String,
    /// Name of the volume.
    #[serde(rename = "Name")]
    pub name: String,
    /// The driver specific options used when creating the volume.
    #[serde(rename = "Options")]
    pub options: std::collections::HashMap<String, String>,
    /// The level at which the volume exists. Either `global` for cluster-wide,
    /// or `local` for machine level.
    #[serde(rename = "Scope")]
    pub scope: String,
    /// Low-level details about the volume, provided by the volume driver.
    /// Details are returned as a map with key/value pairs:
    /// `{"key":"value","key2":"value2"}`.
    ///
    /// The `Status` field is optional, and is omitted if the volume driver
    /// does not support this feature.
    #[serde(rename = "Status")]
    pub status: Option<std::collections::HashMap<String, ()>>,
    #[serde(rename = "UsageData")]
    pub usage_data: Option<crate::v5::models::UsageData>,
}

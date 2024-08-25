use serde::{Deserialize, Serialize};
#[derive(Default, Debug, Serialize, Deserialize)]
/// CreateOptions VolumeConfig
/// Volume configuration
pub struct CreateOptions {
    #[serde(rename = "ClusterVolumeSpec")]
    pub cluster_volume_spec: Option<crate::v5::models::ClusterVolumeSpec>,
    /// Name of the volume driver to use.
    #[serde(rename = "Driver")]
    pub driver: Option<String>,
    /// A mapping of driver options and values. These options are
    /// passed directly to the driver and are driver specific.
    #[serde(rename = "DriverOpts")]
    pub driver_opts: Option<std::collections::HashMap<String, Option<String>>>,
    /// User-defined key/value metadata.
    #[serde(rename = "Labels")]
    pub labels: Option<std::collections::HashMap<String, Option<String>>>,
    /// The new volume's name. If not specified, Docker generates a name.
    #[serde(rename = "Name")]
    pub name: Option<String>,
}

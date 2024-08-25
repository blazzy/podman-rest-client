use serde::{Deserialize, Serialize};
#[derive(Default, Debug, Serialize, Deserialize)]
/// Details for creating a volume
pub struct VolumeCreate {
    /// Name of the volume driver to use.
    #[serde(rename = "Driver")]
    pub driver: String,
    /// A mapping of driver options and values. These options are
    /// passed directly to the driver and are driver specific.
    #[serde(rename = "DriverOpts")]
    pub driver_opts: std::collections::HashMap<String, String>,
    /// User-defined key/value metadata.
    #[serde(rename = "Labels")]
    pub labels: std::collections::HashMap<String, String>,
    /// The new volume's name. If not specified, Docker generates a name.
    #[serde(rename = "Name")]
    pub name: String,
}

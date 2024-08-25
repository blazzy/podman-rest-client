use serde::{Deserialize, Serialize};
#[derive(Default, Debug, Serialize, Deserialize)]
/// VolumeCreateOptions provides details for creating volumes
pub struct VolumeCreateOptions {
    /// Volume driver to use
    #[serde(rename = "Driver")]
    pub driver: Option<String>,
    /// Ignore existing volumes
    #[serde(rename = "IgnoreIfExists")]
    pub ignore_if_exists: Option<bool>,
    /// User-defined key/value metadata. Provided for compatibility
    #[serde(rename = "Label")]
    pub label: Option<std::collections::HashMap<String, Option<String>>>,
    /// User-defined key/value metadata. Preferred field, will override Label
    #[serde(rename = "Labels")]
    pub labels: Option<std::collections::HashMap<String, Option<String>>>,
    /// New volume's name. Can be left blank
    #[serde(rename = "Name")]
    pub name: Option<String>,
    /// Mapping of driver options and values.
    #[serde(rename = "Options")]
    pub options: Option<std::collections::HashMap<String, Option<String>>>,
}

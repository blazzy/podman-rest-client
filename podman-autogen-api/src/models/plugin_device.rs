use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Serialize, Deserialize)]
/// PluginDevice plugin device
pub struct PluginDevice {
    /// description
    #[serde(rename = "Description")]
    pub description: String,

    /// name
    #[serde(rename = "Name")]
    pub name: String,

    /// path
    #[serde(rename = "Path")]
    pub path: String,

    /// settable
    #[serde(rename = "Settable")]
    pub settable: Vec<String>,
}

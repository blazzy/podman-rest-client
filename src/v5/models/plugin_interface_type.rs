use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Serialize, Deserialize)]
/// PluginInterfaceType plugin interface type
pub struct PluginInterfaceType {
    /// capability
    #[serde(rename = "Capability")]
    pub capability: String,

    /// prefix
    #[serde(rename = "Prefix")]
    pub prefix: String,

    /// version
    #[serde(rename = "Version")]
    pub version: String,
}

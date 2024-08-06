use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Serialize, Deserialize)]
/// PluginSettings Settings that can be modified by users.
pub struct PluginSettings {
    /// args
    #[serde(rename = "Args")]
    pub args: Vec<String>,

    /// devices
    #[serde(rename = "Devices")]
    pub devices: Vec<super::super::models::PluginDevice>,

    /// env
    #[serde(rename = "Env")]
    pub env: Vec<String>,

    /// mounts
    #[serde(rename = "Mounts")]
    pub mounts: Vec<super::super::models::PluginMount>,
}

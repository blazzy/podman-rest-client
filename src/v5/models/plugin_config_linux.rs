use serde::{Deserialize, Serialize};
#[derive(Default, Debug, Serialize, Deserialize)]
/// PluginConfigLinux plugin config linux
pub struct PluginConfigLinux {
    /// allow all devices
    #[serde(rename = "AllowAllDevices")]
    pub allow_all_devices: bool,
    /// capabilities
    #[serde(rename = "Capabilities")]
    pub capabilities: Vec<String>,
    /// devices
    #[serde(rename = "Devices")]
    pub devices: Vec<crate::v5::models::PluginDevice>,
}

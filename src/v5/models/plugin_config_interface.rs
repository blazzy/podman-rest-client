use serde::{Deserialize, Serialize};
#[derive(Default, Debug, Serialize, Deserialize)]
/// PluginConfigInterface The interface between Docker and the plugin
pub struct PluginConfigInterface {
    /// Protocol to use for clients connecting to the plugin.
    #[serde(rename = "ProtocolScheme")]
    pub protocol_scheme: Option<String>,
    /// socket
    #[serde(rename = "Socket")]
    pub socket: String,
    /// types
    #[serde(rename = "Types")]
    pub types: Vec<crate::v5::models::PluginInterfaceType>,
}

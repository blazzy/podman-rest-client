use serde::{Deserialize, Serialize};
#[derive(Default, Debug, Serialize, Deserialize)]
/// Secret represents a Swarm Secret value that must be passed to the CSI
/// storage plugin when operating on this Volume. It represents one key-value
/// pair of possibly many.
pub struct Secret {
    /// Key is the name of the key of the key-value pair passed to the plugin.
    #[serde(rename = "Key")]
    pub key: Option<String>,
    /// Secret is the swarm Secret object from which to read data. This can be a
    /// Secret name or ID. The Secret data is retrieved by Swarm and used as the
    /// value of the key-value pair passed to the plugin.
    #[serde(rename = "Secret")]
    pub secret: Option<String>,
}

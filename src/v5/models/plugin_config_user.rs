use serde::{Deserialize, Serialize};
#[derive(Default, Debug, Serialize, Deserialize)]
/// PluginConfigUser plugin config user
pub struct PluginConfigUser {
    /// g ID
    #[serde(rename = "GID")]
    pub gid: Option<u32>,
    /// UID
    #[serde(rename = "UID")]
    pub uid: Option<u32>,
}

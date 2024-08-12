use serde::{Deserialize, Serialize};
#[derive(Default, Debug, Serialize, Deserialize)]
/// PluginConfigRootfs plugin config rootfs
pub struct PluginConfigRootfs {
    /// diff ids
    pub diff_ids: Option<Vec<String>>,
    /// type
    #[serde(rename = "type")]
    pub r#type: Option<String>,
}

use serde::{Deserialize, Serialize};
#[derive(Default, Debug, Serialize, Deserialize)]
/// PluginConfigNetwork plugin config network
pub struct PluginConfigNetwork {
    /// type
    #[serde(rename = "Type")]
    pub r#type: String,
}

use serde::{Deserialize, Serialize};
#[derive(Default, Debug, Serialize, Deserialize)]
/// PluginConfigArgs plugin config args
pub struct PluginConfigArgs {
    /// description
    #[serde(rename = "Description")]
    pub description: String,
    /// name
    #[serde(rename = "Name")]
    pub name: String,
    /// settable
    #[serde(rename = "Settable")]
    pub settable: Vec<String>,
    /// value
    #[serde(rename = "Value")]
    pub value: Vec<String>,
}

use serde::{Deserialize, Serialize};
#[derive(Default, Debug, Serialize, Deserialize)]
/// PluginEnv plugin env
pub struct PluginEnv {
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
    pub value: String,
}

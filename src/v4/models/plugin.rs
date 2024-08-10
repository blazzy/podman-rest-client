use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Serialize, Deserialize)]
/// Plugin A plugin for the Engine API
pub struct Plugin {
    #[serde(rename = "Config")]
    pub config: super::super::models::PluginConfig,

    /// True if the plugin is running. False if the plugin is not running, only installed.
    #[serde(rename = "Enabled")]
    pub enabled: bool,

    /// Id
    #[serde(rename = "Id")]
    pub id: Option<String>,

    /// name
    #[serde(rename = "Name")]
    pub name: String,

    /// plugin remote reference used to push/pull the plugin
    #[serde(rename = "PluginReference")]
    pub plugin_reference: Option<String>,

    #[serde(rename = "Settings")]
    pub settings: super::super::models::PluginSettings,
}

use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Serialize, Deserialize)]
/// PluginMount plugin mount
pub struct PluginMount {
    /// description
    #[serde(rename = "Description")]
    pub description: String,

    /// destination
    #[serde(rename = "Destination")]
    pub destination: String,

    /// name
    #[serde(rename = "Name")]
    pub name: String,

    /// options
    #[serde(rename = "Options")]
    pub options: Vec<String>,

    /// settable
    #[serde(rename = "Settable")]
    pub settable: Vec<String>,

    /// source
    #[serde(rename = "Source")]
    pub source: String,

    /// type
    #[serde(rename = "Type")]
    pub r#type: String,
}

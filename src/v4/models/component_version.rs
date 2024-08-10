use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Serialize, Deserialize)]
/// ComponentVersion describes the version information for a specific component.
pub struct ComponentVersion {
    #[serde(rename = "Details")]
    pub details: Option<std::collections::HashMap<String, String>>,

    #[serde(rename = "Name")]
    pub name: Option<String>,

    #[serde(rename = "Version")]
    pub version: Option<String>,
}

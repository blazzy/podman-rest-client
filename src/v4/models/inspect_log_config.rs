use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Serialize, Deserialize)]
/// InspectLogConfig holds information about a container's configured log driver
pub struct InspectLogConfig {
    #[serde(rename = "Config")]
    pub config: Option<std::collections::HashMap<String, String>>,

    /// Path specifies a path to the log file
    #[serde(rename = "Path")]
    pub path: Option<String>,

    /// Size specifies a maximum size of the container log
    #[serde(rename = "Size")]
    pub size: Option<String>,

    /// Tag specifies a custom log tag for the container
    #[serde(rename = "Tag")]
    pub tag: Option<String>,

    #[serde(rename = "Type")]
    pub r#type: Option<String>,
}

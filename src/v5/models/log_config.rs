use serde::{Deserialize, Serialize};
#[derive(Default, Debug, Serialize, Deserialize)]
/// LogConfig represents the logging configuration of the container.
pub struct LogConfig {
    #[serde(rename = "Config")]
    pub config: Option<std::collections::HashMap<String, Option<String>>>,
    #[serde(rename = "Type")]
    pub r#type: Option<String>,
}

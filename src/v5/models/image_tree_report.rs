use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Serialize, Deserialize)]
pub struct ImageTreeReport {
    #[serde(rename = "Tree")]
    pub tree: Option<String>,
}

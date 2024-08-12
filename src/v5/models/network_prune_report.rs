use serde::{Deserialize, Serialize};
#[derive(Default, Debug, Serialize, Deserialize)]
/// NetworkPruneReport containers the name of network and an error
/// associated in its pruning (removal)
pub struct NetworkPruneReport {
    #[serde(rename = "Error")]
    pub error: Option<String>,
    #[serde(rename = "Name")]
    pub name: Option<String>,
}

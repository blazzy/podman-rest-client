use serde::{Deserialize, Serialize};
#[derive(Default, Debug, Serialize, Deserialize)]
pub struct NetworkPrune200 {
    #[serde(rename = "NetworksDeleted")]
    pub networks_deleted: Option<Vec<String>>,
}

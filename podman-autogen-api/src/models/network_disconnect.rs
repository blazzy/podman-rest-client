use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Serialize, Deserialize)]
/// NetworkDisconnect represents the data to be used to disconnect a container from the network
pub struct NetworkDisconnect {
    #[serde(rename = "Container")]
    pub container: Option<String>,

    #[serde(rename = "Force")]
    pub force: Option<bool>,
}

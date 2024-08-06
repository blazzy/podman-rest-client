use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Serialize, Deserialize)]
/// PeerInfo represents one peer of an overlay network
pub struct PeerInfo {
    #[serde(rename = "IP")]
    pub ip: Option<String>,

    #[serde(rename = "Name")]
    pub name: Option<String>,
}

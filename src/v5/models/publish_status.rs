use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Serialize, Deserialize)]
/// PublishStatus represents the status of the volume as published to an
/// individual node
pub struct PublishStatus {
    /// NodeID is the ID of the swarm node this Volume is published to.
    #[serde(rename = "NodeID")]
    pub node_id: Option<String>,

    /// PublishContext is the PublishContext returned by the CSI plugin when
    /// a volume is published.
    #[serde(rename = "PublishContext")]
    pub publish_context: Option<std::collections::HashMap<String, String>>,

    #[serde(rename = "State")]
    pub state: Option<String>,
}

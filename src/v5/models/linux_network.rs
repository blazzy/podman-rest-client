use serde::{Deserialize, Serialize};
#[derive(Default, Debug, Serialize, Deserialize)]
/// LinuxNetwork identification and priority configuration
pub struct LinuxNetwork {
    /// Set class identifier for container's network packets
    #[serde(rename = "classID")]
    pub class_id: Option<u32>,
    /// Set priority of network traffic for container
    pub priorities: Option<Vec<super::super::models::LinuxInterfacePriority>>,
}

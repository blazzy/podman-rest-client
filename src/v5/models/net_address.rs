use serde::{Deserialize, Serialize};
#[derive(Default, Debug, Serialize, Deserialize)]
/// NetAddress contains the ip address, subnet and gateway.
pub struct NetAddress {
    /// Gateway for the network. This can be empty if there is no gateway, e.g. internal network.
    pub gateway: Option<String>,
    pub ipnet: Option<crate::v5::models::IpNet>,
}

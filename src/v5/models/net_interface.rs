use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Serialize, Deserialize)]
/// NetInterface contains the settings for a given network interface.
pub struct NetInterface {
    /// MacAddress for this Interface.
    pub mac_address: Option<String>,

    /// Subnets list of assigned subnets with their gateway.
    pub subnets: Option<Vec<super::super::models::NetAddress>>,
}

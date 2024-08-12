use serde::{Deserialize, Serialize};
#[derive(Default, Debug, Serialize, Deserialize)]
/// EndpointResource contains network resources allocated and used for a container in a network
pub struct EndpointResource {
    #[serde(rename = "EndpointID")]
    pub endpoint_id: Option<String>,
    #[serde(rename = "IPv4Address")]
    pub i_pv_4_address: Option<String>,
    #[serde(rename = "IPv6Address")]
    pub i_pv_6_address: Option<String>,
    #[serde(rename = "MacAddress")]
    pub mac_address: Option<String>,
    #[serde(rename = "Name")]
    pub name: Option<String>,
}

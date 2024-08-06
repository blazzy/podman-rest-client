use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Serialize, Deserialize)]
/// EndpointIPAMConfig represents IPAM configurations for the endpoint
pub struct EndpointIpamConfig {
    #[serde(rename = "IPv4Address")]
    pub i_pv_4_address: Option<String>,

    #[serde(rename = "IPv6Address")]
    pub i_pv_6_address: Option<String>,

    #[serde(rename = "LinkLocalIPs")]
    pub link_local_i_ps: Option<Vec<String>>,
}

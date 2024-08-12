use serde::{Deserialize, Serialize};
#[derive(Default, Debug, Serialize, Deserialize)]
/// EndpointSettings stores the network endpoint details
pub struct EndpointSettings {
    #[serde(rename = "Aliases")]
    pub aliases: Option<Vec<String>>,
    /// DNSNames holds all the (non fully qualified) DNS names associated to this endpoint. First entry is used to
    /// generate PTR records.
    #[serde(rename = "DNSNames")]
    pub dns_names: Option<Vec<String>>,
    #[serde(rename = "DriverOpts")]
    pub driver_opts: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "EndpointID")]
    pub endpoint_id: Option<String>,
    #[serde(rename = "Gateway")]
    pub gateway: Option<String>,
    #[serde(rename = "GlobalIPv6Address")]
    pub global_i_pv_6_address: Option<String>,
    #[serde(rename = "GlobalIPv6PrefixLen")]
    pub global_i_pv_6_prefix_len: Option<i64>,
    #[serde(rename = "IPAMConfig")]
    pub ipam_config: Option<super::super::models::EndpointIpamConfig>,
    #[serde(rename = "IPAddress")]
    pub ip_address: Option<String>,
    #[serde(rename = "IPPrefixLen")]
    pub ip_prefix_len: Option<i64>,
    #[serde(rename = "IPv6Gateway")]
    pub i_pv_6_gateway: Option<String>,
    #[serde(rename = "Links")]
    pub links: Option<Vec<String>>,
    /// MacAddress may be used to specify a MAC address when the container is created.
    /// Once the container is running, it becomes operational data (it may contain a
    /// generated address).
    #[serde(rename = "MacAddress")]
    pub mac_address: Option<String>,
    /// Operational data
    #[serde(rename = "NetworkID")]
    pub network_id: Option<String>,
}

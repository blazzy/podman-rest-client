use serde::{Deserialize, Serialize};
#[derive(Default, Debug, Serialize, Deserialize)]
/// NetworkSettings exposes the network settings in the api
pub struct NetworkSettings {
    #[serde(rename = "Bridge")]
    pub bridge: Option<String>,
    #[serde(rename = "EndpointID")]
    pub endpoint_id: Option<String>,
    #[serde(rename = "Gateway")]
    pub gateway: Option<String>,
    #[serde(rename = "GlobalIPv6Address")]
    pub global_i_pv6_address: Option<String>,
    #[serde(rename = "GlobalIPv6PrefixLen")]
    pub global_i_pv6_prefix_len: Option<i64>,
    /// HairpinMode specifies if hairpin NAT should be enabled on the virtual interface
    ///
    /// Deprecated: This field is never set and will be removed in a future release.
    #[serde(rename = "HairpinMode")]
    pub hairpin_mode: Option<bool>,
    #[serde(rename = "IPAddress")]
    pub ip_address: Option<String>,
    #[serde(rename = "IPPrefixLen")]
    pub ip_prefix_len: Option<i64>,
    #[serde(rename = "IPv6Gateway")]
    pub i_pv6_gateway: Option<String>,
    /// LinkLocalIPv6Address is an IPv6 unicast address using the link-local prefix
    ///
    /// Deprecated: This field is never set and will be removed in a future release.
    #[serde(rename = "LinkLocalIPv6Address")]
    pub link_local_i_pv6_address: Option<String>,
    /// LinkLocalIPv6PrefixLen is the prefix length of an IPv6 unicast address
    ///
    /// Deprecated: This field is never set and will be removed in a future release.
    #[serde(rename = "LinkLocalIPv6PrefixLen")]
    pub link_local_i_pv6_prefix_len: Option<i64>,
    #[serde(rename = "MacAddress")]
    pub mac_address: Option<String>,
    #[serde(rename = "Networks")]
    pub networks: Option<std::collections::HashMap<String, crate::v5::models::EndpointSettings>>,
    #[serde(rename = "Ports")]
    pub ports: Option<std::collections::HashMap<String, Vec<crate::v5::models::PortBinding>>>,
    #[serde(rename = "SandboxID")]
    pub sandbox_id: Option<String>,
    #[serde(rename = "SandboxKey")]
    pub sandbox_key: Option<String>,
    #[serde(rename = "SecondaryIPAddresses")]
    pub secondary_ip_addresses: Option<Vec<crate::v5::models::Address>>,
    #[serde(rename = "SecondaryIPv6Addresses")]
    pub secondary_i_pv6_addresses: Option<Vec<crate::v5::models::Address>>,
}

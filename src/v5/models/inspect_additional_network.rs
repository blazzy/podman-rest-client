use serde::{Deserialize, Serialize};
#[derive(Default, Debug, Serialize, Deserialize)]
/// InspectAdditionalNetwork holds information about non-default networks the
/// container has been connected to.
/// As with InspectNetworkSettings, many fields are unused and maintained only
/// for compatibility with Docker.
pub struct InspectAdditionalNetwork {
    /// AdditionalMacAddresses is a set of additional MAC Addresses beyond
    /// the first. CNI may configure more than one interface for a single
    /// network, which can cause this.
    #[serde(rename = "AdditionalMACAddresses")]
    pub additional_mac_addresses: Option<Vec<String>>,
    /// Aliases are any network aliases the container has in this network.
    #[serde(rename = "Aliases")]
    pub aliases: Option<Vec<String>>,
    /// DriverOpts is presently unused and maintained exclusively for
    /// compatibility.
    #[serde(rename = "DriverOpts")]
    pub driver_opts: Option<std::collections::HashMap<String, Option<String>>>,
    /// EndpointID is unused, maintained exclusively for compatibility.
    #[serde(rename = "EndpointID")]
    pub endpoint_id: Option<String>,
    /// Gateway is the IP address of the gateway this network will use.
    #[serde(rename = "Gateway")]
    pub gateway: Option<String>,
    /// GlobalIPv6Address is the global-scope IPv6 Address for this network.
    #[serde(rename = "GlobalIPv6Address")]
    pub global_i_pv6_address: Option<String>,
    /// GlobalIPv6PrefixLen is the length of the subnet mask of this network.
    #[serde(rename = "GlobalIPv6PrefixLen")]
    pub global_i_pv6_prefix_len: Option<i64>,
    /// IPAMConfig is presently unused and maintained exclusively for
    /// compatibility.
    #[serde(rename = "IPAMConfig")]
    pub ipam_config: Option<std::collections::HashMap<String, Option<String>>>,
    /// IPAddress is the IP address for this network.
    #[serde(rename = "IPAddress")]
    pub ip_address: Option<String>,
    /// IPPrefixLen is the length of the subnet mask of this network.
    #[serde(rename = "IPPrefixLen")]
    pub ip_prefix_len: Option<i64>,
    /// IPv6Gateway is the IPv6 gateway this network will use.
    #[serde(rename = "IPv6Gateway")]
    pub i_pv6_gateway: Option<String>,
    /// Links is presently unused and maintained exclusively for
    /// compatibility.
    #[serde(rename = "Links")]
    pub links: Option<Vec<String>>,
    /// MacAddress is the MAC address for the interface in this network.
    #[serde(rename = "MacAddress")]
    pub mac_address: Option<String>,
    /// Name of the network we're connecting to.
    #[serde(rename = "NetworkID")]
    pub network_id: Option<String>,
    /// SecondaryIPAddresses is a list of extra IP Addresses that the
    /// container has been assigned in this network.
    #[serde(rename = "SecondaryIPAddresses")]
    pub secondary_ip_addresses: Option<Vec<crate::v5::models::Address>>,
    /// SecondaryIPv6Addresses is a list of extra IPv6 Addresses that the
    /// container has been assigned in this network.
    #[serde(rename = "SecondaryIPv6Addresses")]
    pub secondary_i_pv6_addresses: Option<Vec<crate::v5::models::Address>>,
}

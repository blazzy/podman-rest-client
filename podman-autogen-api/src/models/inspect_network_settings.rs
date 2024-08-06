use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Serialize, Deserialize)]
/// InspectNetworkSettings holds information about the network settings of the
/// container.
/// Many fields are maintained only for compatibility with `docker inspect` and
/// are unused within Libpod.
pub struct InspectNetworkSettings {
    /// AdditionalMacAddresses is a set of additional MAC Addresses beyond
    /// the first. CNI may configure more than one interface for a single
    /// network, which can cause this.
    #[serde(rename = "AdditionalMACAddresses")]
    pub additional_mac_addresses: Option<Vec<String>>,

    #[serde(rename = "Bridge")]
    pub bridge: Option<String>,

    /// EndpointID is unused, maintained exclusively for compatibility.
    #[serde(rename = "EndpointID")]
    pub endpoint_id: Option<String>,

    /// Gateway is the IP address of the gateway this network will use.
    #[serde(rename = "Gateway")]
    pub gateway: Option<String>,

    /// GlobalIPv6Address is the global-scope IPv6 Address for this network.
    #[serde(rename = "GlobalIPv6Address")]
    pub global_i_pv_6_address: Option<String>,

    /// GlobalIPv6PrefixLen is the length of the subnet mask of this network.
    #[serde(rename = "GlobalIPv6PrefixLen")]
    pub global_i_pv_6_prefix_len: Option<i64>,

    #[serde(rename = "HairpinMode")]
    pub hairpin_mode: Option<bool>,

    /// IPAddress is the IP address for this network.
    #[serde(rename = "IPAddress")]
    pub ip_address: Option<String>,

    /// IPPrefixLen is the length of the subnet mask of this network.
    #[serde(rename = "IPPrefixLen")]
    pub ip_prefix_len: Option<i64>,

    /// IPv6Gateway is the IPv6 gateway this network will use.
    #[serde(rename = "IPv6Gateway")]
    pub i_pv_6_gateway: Option<String>,

    #[serde(rename = "LinkLocalIPv6Address")]
    pub link_local_i_pv_6_address: Option<String>,

    #[serde(rename = "LinkLocalIPv6PrefixLen")]
    pub link_local_i_pv_6_prefix_len: Option<i64>,

    /// MacAddress is the MAC address for the interface in this network.
    #[serde(rename = "MacAddress")]
    pub mac_address: Option<String>,

    /// Networks contains information on non-default networks this
    /// container has joined.
    /// It is a map of network name to network information.
    #[serde(rename = "Networks")]
    pub networks:
        Option<std::collections::HashMap<String, super::super::models::InspectAdditionalNetwork>>,

    #[serde(rename = "Ports")]
    pub ports: Option<
        std::collections::HashMap<String, Option<Vec<super::super::models::InspectHostPort>>>,
    >,

    #[serde(rename = "SandboxID")]
    pub sandbox_id: Option<String>,

    #[serde(rename = "SandboxKey")]
    pub sandbox_key: Option<String>,

    /// SecondaryIPAddresses is a list of extra IP Addresses that the
    /// container has been assigned in this network.
    #[serde(rename = "SecondaryIPAddresses")]
    pub secondary_ip_addresses: Option<Vec<super::super::models::Address>>,

    /// SecondaryIPv6Addresses is a list of extra IPv6 Addresses that the
    /// container has been assigned in this network.
    #[serde(rename = "SecondaryIPv6Addresses")]
    pub secondary_i_pv_6_addresses: Option<Vec<super::super::models::Address>>,
}

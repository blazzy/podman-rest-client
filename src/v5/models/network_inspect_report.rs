use serde::{Deserialize, Serialize};
#[derive(Default, Debug, Serialize, Deserialize)]
pub struct NetworkInspectReport {
    pub containers:
        Option<std::collections::HashMap<String, super::super::models::NetworkContainerInfo>>,
    /// Created contains the timestamp when this network was created.
    pub created: Option<String>,
    /// DNSEnabled is whether name resolution is active for container on
    /// this Network. Only supported with the bridge driver.
    pub dns_enabled: Option<bool>,
    /// Driver for this Network, e.g. bridge, macvlan...
    pub driver: Option<String>,
    /// ID of the Network.
    pub id: Option<String>,
    /// Internal is whether the Network should not have external routes
    /// to public or other Networks.
    pub internal: Option<bool>,
    /// IPAMOptions contains options used for the ip assignment.
    pub ipam_options: Option<std::collections::HashMap<String, String>>,
    /// IPv6Enabled if set to true an ipv6 subnet should be created for this net.
    #[serde(rename = "ipv6_enabled")]
    pub ipv_6_enabled: Option<bool>,
    /// Labels is a set of key-value labels that have been applied to the
    /// Network.
    pub labels: Option<std::collections::HashMap<String, String>>,
    /// Name of the Network.
    pub name: Option<String>,
    /// List of custom DNS server for podman's DNS resolver at network level,
    /// all the containers attached to this network will consider resolvers
    /// configured at network level.
    pub network_dns_servers: Option<Vec<String>>,
    /// NetworkInterface is the network interface name on the host.
    pub network_interface: Option<String>,
    /// Options is a set of key-value options that have been applied to
    /// the Network.
    pub options: Option<std::collections::HashMap<String, String>>,
    /// Routes to use for this network.
    pub routes: Option<Vec<super::super::models::Route>>,
    /// Subnets to use for this network.
    pub subnets: Option<Vec<super::super::models::Subnet>>,
}

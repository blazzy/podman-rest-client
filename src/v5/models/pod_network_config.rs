use serde::{Deserialize, Serialize};
#[derive(Default, Debug, Serialize, Deserialize)]
/// PodNetworkConfig contains networking configuration for a pod.
pub struct PodNetworkConfig {
    /// Map of networks names to ids the container should join to.
    /// You can request additional settings for each network, you can
    /// set network aliases, static ips, static mac address  and the
    /// network interface name for this container on the specific network.
    /// If the map is empty and the bridge network mode is set the container
    /// will be joined to the default network.
    #[serde(rename = "Networks")]
    pub networks:
        Option<std::collections::HashMap<String, super::super::models::PerNetworkOptions>>,
    /// CNINetworks is a list of CNI networks to join the container to.
    /// If this list is empty, the default CNI network will be joined
    /// instead. If at least one entry is present, we will not join the
    /// default network (unless it is part of this list).
    /// Only available if NetNS is set to bridge.
    /// Optional.
    /// Deprecated: as of podman 4.0 use "Networks" instead.
    pub cni_networks: Option<Vec<String>>,
    /// DNSOption is a set of DNS options that will be used in the infra
    /// container's resolv.conf, which will, by default, be shared with all
    /// containers in the pod.
    /// Conflicts with NoInfra=true.
    /// Optional.
    pub dns_option: Option<Vec<String>>,
    /// DNSSearch is a set of DNS search domains that will be used in the
    /// infra container's resolv.conf, which will, by default, be shared with
    /// all containers in the pod.
    /// If not provided, DNS search domains from the host's resolv.conf will
    /// be used.
    /// Conflicts with NoInfra=true.
    /// Optional.
    pub dns_search: Option<Vec<String>>,
    /// DNSServer is a set of DNS servers that will be used in the infra
    /// container's resolv.conf, which will, by default, be shared with all
    /// containers in the pod.
    /// If not provided, the host's DNS servers will be used, unless the only
    /// server set is a localhost address. As the container cannot connect to
    /// the host's localhost, a default server will instead be set.
    /// Conflicts with NoInfra=true.
    /// Optional.
    pub dns_server: Option<Vec<String>>,
    /// HostAdd is a set of hosts that will be added to the infra container's
    /// etc/hosts that will, by default, be shared with all containers in
    /// the pod.
    /// Conflicts with NoInfra=true and NoManageHosts.
    /// Optional.
    pub hostadd: Option<Vec<String>>,
    pub netns: Option<super::super::models::Namespace>,
    /// NetworkOptions are additional options for each network
    /// Optional.
    pub network_options: Option<std::collections::HashMap<String, Vec<String>>>,
    /// NoManageHosts indicates that /etc/hosts should not be managed by the
    /// pod. Instead, each container will create a separate /etc/hosts as
    /// they would if not in a pod.
    /// Conflicts with HostAdd.
    pub no_manage_hosts: Option<bool>,
    /// NoManageResolvConf indicates that /etc/resolv.conf should not be
    /// managed by the pod. Instead, each container will create and manage a
    /// separate resolv.conf as if they had not joined a pod.
    /// Conflicts with NoInfra=true and DNSServer, DNSSearch, DNSOption.
    /// Optional.
    pub no_manage_resolv_conf: Option<bool>,
    /// PortMappings is a set of ports to map into the infra container.
    /// As, by default, containers share their network with the infra
    /// container, this will forward the ports to the entire pod.
    /// Only available if NetNS is set to Bridge, Slirp, or Pasta.
    /// Optional.
    pub portmappings: Option<Vec<super::super::models::PortMapping>>,
}

use serde::{Deserialize, Serialize};
#[derive(Default, Debug, Serialize, Deserialize)]
/// InspectPodInfraConfig contains the configuration of the pod's infra
/// container.
pub struct InspectPodInfraConfig {
    /// DNSOption is a set of DNS options that will be used by the infra
    /// container's resolv.conf and shared with the remainder of the pod.
    #[serde(rename = "DNSOption")]
    pub dns_option: Option<Vec<String>>,
    /// DNSSearch is a set of DNS search domains that will be used by the
    /// infra container's resolv.conf and shared with the remainder of the
    /// pod.
    #[serde(rename = "DNSSearch")]
    pub dns_search: Option<Vec<String>>,
    /// DNSServer is a set of DNS Servers that will be used by the infra
    /// container's resolv.conf and shared with the remainder of the pod.
    #[serde(rename = "DNSServer")]
    pub dns_server: Option<Vec<String>>,
    /// HostAdd adds a number of hosts to the infra container's resolv.conf
    /// which will be shared with the rest of the pod.
    #[serde(rename = "HostAdd")]
    pub host_add: Option<Vec<String>>,
    /// HostNetwork is whether the infra container (and thus the whole pod)
    /// will use the host's network and not create a network namespace.
    #[serde(rename = "HostNetwork")]
    pub host_network: Option<bool>,
    /// NetworkOptions are additional options for each network
    #[serde(rename = "NetworkOptions")]
    pub network_options: Option<std::collections::HashMap<String, Vec<String>>>,
    /// Networks is a list of networks the pod will join.
    #[serde(rename = "Networks")]
    pub networks: Option<Vec<String>>,
    /// NoManageHosts indicates that the pod will not manage /etc/hosts and
    /// instead each container will handle their own.
    #[serde(rename = "NoManageHosts")]
    pub no_manage_hosts: Option<bool>,
    /// NoManageResolvConf indicates that the pod will not manage resolv.conf
    /// and instead each container will handle their own.
    #[serde(rename = "NoManageResolvConf")]
    pub no_manage_resolv_conf: Option<bool>,
    /// PortBindings are ports that will be forwarded to the infra container
    /// and then shared with the pod.
    #[serde(rename = "PortBindings")]
    pub port_bindings: Option<
        std::collections::HashMap<String, Option<Vec<super::super::models::InspectHostPort>>>,
    >,
    /// StaticIP is a static IPv4 that will be assigned to the infra
    /// container and then used by the pod.
    #[serde(rename = "StaticIP")]
    pub static_ip: Option<String>,
    /// StaticMAC is a static MAC address that will be assigned to the infra
    /// container and then used by the pod.
    #[serde(rename = "StaticMAC")]
    pub static_mac: Option<String>,
    /// CPUPeriod contains the CPU period of the pod
    pub cpu_period: Option<u64>,
    /// CPUQuota contains the CPU quota of the pod
    pub cpu_quota: Option<i64>,
    /// CPUSetCPUs contains linux specific CPU data for the container
    pub cpuset_cpus: Option<String>,
    /// Pid is the PID namespace mode of the pod's infra container
    pub pid_ns: Option<String>,
    /// UserNS is the usernamespace that all the containers in the pod will join.
    pub userns: Option<String>,
    /// UtsNS is the uts namespace that all containers in the pod will join
    pub uts_ns: Option<String>,
}

/*
 * supports a RESTful API for the Libpod library
 *
 * This documentation describes the Podman v2.x+ RESTful API. It consists of a Docker-compatible API and a Libpod API providing support for Podman’s unique features such as pods.  To start the service and keep it running for 5,000 seconds (-t 0 runs forever):  podman system service -t 5000 &  You can then use cURL on the socket using requests documented below.  NOTE: if you install the package podman-docker, it will create a symbolic link for /run/docker.sock to /run/podman/podman.sock  NOTE: Some fields in the API response JSON are encoded as omitempty, which means that if said field has a zero value, they will not be encoded in the API response. This is a feature to help reduce the size of the JSON responses returned via the API.  NOTE: Due to the limitations of [go-swagger](https://github.com/go-swagger/go-swagger), some field values that have a complex type show up as null in the docs as well as in the API responses. This is because the zero value for the field type is null. The field description in the docs will state what type the field is expected to be for such cases.  See podman-system-service(1) for more information.  Quick Examples:  'podman info'  curl --unix-socket /run/podman/podman.sock http://d/v5.0.0/libpod/info  'podman pull quay.io/containers/podman'  curl -XPOST --unix-socket /run/podman/podman.sock -v 'http://d/v5.0.0/images/create?fromImage=quay.io%2Fcontainers%2Fpodman'  'podman list images'  curl --unix-socket /run/podman/podman.sock -v 'http://d/v5.0.0/libpod/images/json' | jq
 *
 * The version of the OpenAPI document: 5.0.0
 * Contact: podman@lists.podman.io
 * Generated by: https://openapi-generator.tech
 */

use crate::models;
use serde::{Deserialize, Serialize};

/// InspectPodInfraConfig : InspectPodInfraConfig contains the configuration of the pod's infra container.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct InspectPodInfraConfig {
    /// DNSOption is a set of DNS options that will be used by the infra container's resolv.conf and shared with the remainder of the pod.
    #[serde(rename = "DNSOption", skip_serializing_if = "Option::is_none")]
    pub dns_option: Option<Vec<String>>,
    /// DNSSearch is a set of DNS search domains that will be used by the infra container's resolv.conf and shared with the remainder of the pod.
    #[serde(rename = "DNSSearch", skip_serializing_if = "Option::is_none")]
    pub dns_search: Option<Vec<String>>,
    /// DNSServer is a set of DNS Servers that will be used by the infra container's resolv.conf and shared with the remainder of the pod.
    #[serde(rename = "DNSServer", skip_serializing_if = "Option::is_none")]
    pub dns_server: Option<Vec<String>>,
    /// HostAdd adds a number of hosts to the infra container's resolv.conf which will be shared with the rest of the pod.
    #[serde(rename = "HostAdd", skip_serializing_if = "Option::is_none")]
    pub host_add: Option<Vec<String>>,
    /// HostNetwork is whether the infra container (and thus the whole pod) will use the host's network and not create a network namespace.
    #[serde(rename = "HostNetwork", skip_serializing_if = "Option::is_none")]
    pub host_network: Option<bool>,
    /// NetworkOptions are additional options for each network
    #[serde(rename = "NetworkOptions", skip_serializing_if = "Option::is_none")]
    pub network_options: Option<std::collections::HashMap<String, Vec<String>>>,
    /// Networks is a list of networks the pod will join.
    #[serde(rename = "Networks", skip_serializing_if = "Option::is_none")]
    pub networks: Option<Vec<String>>,
    /// NoManageHosts indicates that the pod will not manage /etc/hosts and instead each container will handle their own.
    #[serde(rename = "NoManageHosts", skip_serializing_if = "Option::is_none")]
    pub no_manage_hosts: Option<bool>,
    /// NoManageResolvConf indicates that the pod will not manage resolv.conf and instead each container will handle their own.
    #[serde(rename = "NoManageResolvConf", skip_serializing_if = "Option::is_none")]
    pub no_manage_resolv_conf: Option<bool>,
    /// PortBindings are ports that will be forwarded to the infra container and then shared with the pod.
    #[serde(rename = "PortBindings", skip_serializing_if = "Option::is_none")]
    pub port_bindings: Option<std::collections::HashMap<String, Vec<models::InspectHostPort>>>,
    /// StaticIP is a static IPv4 that will be assigned to the infra container and then used by the pod.
    #[serde(rename = "StaticIP", skip_serializing_if = "Option::is_none")]
    pub static_ip: Option<String>,
    /// StaticMAC is a static MAC address that will be assigned to the infra container and then used by the pod.
    #[serde(rename = "StaticMAC", skip_serializing_if = "Option::is_none")]
    pub static_mac: Option<String>,
    /// CPUPeriod contains the CPU period of the pod
    #[serde(rename = "cpu_period", skip_serializing_if = "Option::is_none")]
    pub cpu_period: Option<i32>,
    /// CPUQuota contains the CPU quota of the pod
    #[serde(rename = "cpu_quota", skip_serializing_if = "Option::is_none")]
    pub cpu_quota: Option<i64>,
    /// CPUSetCPUs contains linux specific CPU data for the container
    #[serde(rename = "cpuset_cpus", skip_serializing_if = "Option::is_none")]
    pub cpuset_cpus: Option<String>,
    /// Pid is the PID namespace mode of the pod's infra container
    #[serde(rename = "pid_ns", skip_serializing_if = "Option::is_none")]
    pub pid_ns: Option<String>,
    /// UserNS is the usernamespace that all the containers in the pod will join.
    #[serde(rename = "userns", skip_serializing_if = "Option::is_none")]
    pub userns: Option<String>,
    /// UtsNS is the uts namespace that all containers in the pod will join
    #[serde(rename = "uts_ns", skip_serializing_if = "Option::is_none")]
    pub uts_ns: Option<String>,
}

impl InspectPodInfraConfig {
    /// InspectPodInfraConfig contains the configuration of the pod's infra container.
    pub fn new() -> InspectPodInfraConfig {
        InspectPodInfraConfig {
            dns_option: None,
            dns_search: None,
            dns_server: None,
            host_add: None,
            host_network: None,
            network_options: None,
            networks: None,
            no_manage_hosts: None,
            no_manage_resolv_conf: None,
            port_bindings: None,
            static_ip: None,
            static_mac: None,
            cpu_period: None,
            cpu_quota: None,
            cpuset_cpus: None,
            pid_ns: None,
            userns: None,
            uts_ns: None,
        }
    }
}


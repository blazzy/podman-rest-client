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

/// PodSpecGenerator : PodSpecGenerator describes options to create a pod
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct PodSpecGenerator {
    /// Map of networks names to ids the container should join to. You can request additional settings for each network, you can set network aliases, static ips, static mac address  and the network interface name for this container on the specific network. If the map is empty and the bridge network mode is set the container will be joined to the default network.
    #[serde(rename = "Networks", skip_serializing_if = "Option::is_none")]
    pub networks: Option<std::collections::HashMap<String, models::PerNetworkOptions>>,
    /// CgroupParent is the parent for the Cgroup that the pod will create. This pod cgroup will, in turn, be the default cgroup parent for all containers in the pod. Optional.
    #[serde(rename = "cgroup_parent", skip_serializing_if = "Option::is_none")]
    pub cgroup_parent: Option<String>,
    /// CNINetworks is a list of CNI networks to join the container to. If this list is empty, the default CNI network will be joined instead. If at least one entry is present, we will not join the default network (unless it is part of this list). Only available if NetNS is set to bridge. Optional. Deprecated: as of podman 4.0 use \"Networks\" instead.
    #[serde(rename = "cni_networks", skip_serializing_if = "Option::is_none")]
    pub cni_networks: Option<Vec<String>>,
    /// CPU period of the cpuset, determined by --cpus
    #[serde(rename = "cpu_period", skip_serializing_if = "Option::is_none")]
    pub cpu_period: Option<i32>,
    /// CPU quota of the cpuset, determined by --cpus
    #[serde(rename = "cpu_quota", skip_serializing_if = "Option::is_none")]
    pub cpu_quota: Option<i64>,
    /// DNSOption is a set of DNS options that will be used in the infra container's resolv.conf, which will, by default, be shared with all containers in the pod. Conflicts with NoInfra=true. Optional.
    #[serde(rename = "dns_option", skip_serializing_if = "Option::is_none")]
    pub dns_option: Option<Vec<String>>,
    /// DNSSearch is a set of DNS search domains that will be used in the infra container's resolv.conf, which will, by default, be shared with all containers in the pod. If not provided, DNS search domains from the host's resolv.conf will be used. Conflicts with NoInfra=true. Optional.
    #[serde(rename = "dns_search", skip_serializing_if = "Option::is_none")]
    pub dns_search: Option<Vec<String>>,
    /// DNSServer is a set of DNS servers that will be used in the infra container's resolv.conf, which will, by default, be shared with all containers in the pod. If not provided, the host's DNS servers will be used, unless the only server set is a localhost address. As the container cannot connect to the host's localhost, a default server will instead be set. Conflicts with NoInfra=true. Optional.
    #[serde(rename = "dns_server", skip_serializing_if = "Option::is_none")]
    pub dns_server: Option<Vec<String>>,
    /// ExitPolicy determines the pod's exit and stop behaviour.
    #[serde(rename = "exit_policy", skip_serializing_if = "Option::is_none")]
    pub exit_policy: Option<String>,
    /// HostAdd is a set of hosts that will be added to the infra container's etc/hosts that will, by default, be shared with all containers in the pod. Conflicts with NoInfra=true and NoManageHosts. Optional.
    #[serde(rename = "hostadd", skip_serializing_if = "Option::is_none")]
    pub hostadd: Option<Vec<String>>,
    /// Hostname is the pod's hostname. If not set, the name of the pod will be used (if a name was not provided here, the name auto-generated for the pod will be used). This will be used by the infra container and all containers in the pod as long as the UTS namespace is shared. Optional.
    #[serde(rename = "hostname", skip_serializing_if = "Option::is_none")]
    pub hostname: Option<String>,
    #[serde(rename = "idmappings", skip_serializing_if = "Option::is_none")]
    pub idmappings: Option<Box<models::IdMappingOptions>>,
    /// Image volumes bind-mount a container-image mount into the pod's infra container. Optional.
    #[serde(rename = "image_volumes", skip_serializing_if = "Option::is_none")]
    pub image_volumes: Option<Vec<models::ImageVolume>>,
    /// InfraCommand sets the command that will be used to start the infra container. If not set, the default set in the Libpod configuration file will be used. Conflicts with NoInfra=true. Optional.
    #[serde(rename = "infra_command", skip_serializing_if = "Option::is_none")]
    pub infra_command: Option<Vec<String>>,
    /// InfraConmonPidFile is a custom path to store the infra container's conmon PID.
    #[serde(
        rename = "infra_conmon_pid_file",
        skip_serializing_if = "Option::is_none"
    )]
    pub infra_conmon_pid_file: Option<String>,
    /// InfraImage is the image that will be used for the infra container. If not set, the default set in the Libpod configuration file will be used. Conflicts with NoInfra=true. Optional.
    #[serde(rename = "infra_image", skip_serializing_if = "Option::is_none")]
    pub infra_image: Option<String>,
    /// InfraName is the name that will be used for the infra container. If not set, the default set in the Libpod configuration file will be used. Conflicts with NoInfra=true. Optional.
    #[serde(rename = "infra_name", skip_serializing_if = "Option::is_none")]
    pub infra_name: Option<String>,
    #[serde(rename = "ipcns", skip_serializing_if = "Option::is_none")]
    pub ipcns: Option<Box<models::Namespace>>,
    /// Labels are key-value pairs that are used to add metadata to pods. Optional.
    #[serde(rename = "labels", skip_serializing_if = "Option::is_none")]
    pub labels: Option<std::collections::HashMap<String, String>>,
    /// Mounts are mounts that will be added to the pod. These will supersede Image Volumes and VolumesFrom volumes where there are conflicts. Optional.
    #[serde(rename = "mounts", skip_serializing_if = "Option::is_none")]
    pub mounts: Option<Vec<models::Mount>>,
    /// Name is the name of the pod. If not provided, a name will be generated when the pod is created. Optional.
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "netns", skip_serializing_if = "Option::is_none")]
    pub netns: Option<Box<models::Namespace>>,
    /// NetworkOptions are additional options for each network Optional.
    #[serde(rename = "network_options", skip_serializing_if = "Option::is_none")]
    pub network_options: Option<std::collections::HashMap<String, Vec<String>>>,
    /// NoInfra tells the pod not to create an infra container. If this is done, many networking-related options will become unavailable. Conflicts with setting any options in PodNetworkConfig, and the InfraCommand and InfraImages in this struct. Optional.
    #[serde(rename = "no_infra", skip_serializing_if = "Option::is_none")]
    pub no_infra: Option<bool>,
    /// NoManageHosts indicates that /etc/hosts should not be managed by the pod. Instead, each container will create a separate /etc/hosts as they would if not in a pod. Conflicts with HostAdd.
    #[serde(rename = "no_manage_hosts", skip_serializing_if = "Option::is_none")]
    pub no_manage_hosts: Option<bool>,
    /// NoManageResolvConf indicates that /etc/resolv.conf should not be managed by the pod. Instead, each container will create and manage a separate resolv.conf as if they had not joined a pod. Conflicts with NoInfra=true and DNSServer, DNSSearch, DNSOption. Optional.
    #[serde(
        rename = "no_manage_resolv_conf",
        skip_serializing_if = "Option::is_none"
    )]
    pub no_manage_resolv_conf: Option<bool>,
    /// Overlay volumes are named volumes that will be added to the pod. Optional.
    #[serde(rename = "overlay_volumes", skip_serializing_if = "Option::is_none")]
    pub overlay_volumes: Option<Vec<models::OverlayVolume>>,
    #[serde(rename = "pidns", skip_serializing_if = "Option::is_none")]
    pub pidns: Option<Box<models::Namespace>>,
    #[serde(rename = "pod_create_command", skip_serializing_if = "Option::is_none")]
    pub pod_create_command: Option<Vec<String>>,
    /// Devices contains user specified Devices to be added to the Pod
    #[serde(rename = "pod_devices", skip_serializing_if = "Option::is_none")]
    pub pod_devices: Option<Vec<String>>,
    /// PortMappings is a set of ports to map into the infra container. As, by default, containers share their network with the infra container, this will forward the ports to the entire pod. Only available if NetNS is set to Bridge, Slirp, or Pasta. Optional.
    #[serde(rename = "portmappings", skip_serializing_if = "Option::is_none")]
    pub portmappings: Option<Vec<models::PortMapping>>,
    #[serde(rename = "resource_limits", skip_serializing_if = "Option::is_none")]
    pub resource_limits: Option<Box<models::LinuxResources>>,
    /// RestartPolicy is the pod's restart policy - an action which will be taken when one or all the containers in the pod exits. If not given, the default policy will be set to Always, which restarts the containers in the pod when they exit indefinitely. Optional.
    #[serde(rename = "restart_policy", skip_serializing_if = "Option::is_none")]
    pub restart_policy: Option<String>,
    /// RestartRetries is the number of attempts that will be made to restart the container. Only available when RestartPolicy is set to \"on-failure\". Optional.
    #[serde(rename = "restart_tries", skip_serializing_if = "Option::is_none")]
    pub restart_tries: Option<i32>,
    #[serde(rename = "security_opt", skip_serializing_if = "Option::is_none")]
    pub security_opt: Option<Vec<String>>,
    /// The ID of the pod's service container.
    #[serde(rename = "serviceContainerID", skip_serializing_if = "Option::is_none")]
    pub service_container_id: Option<String>,
    /// PodCreateCommand is the command used to create this pod. This will be shown in the output of Inspect() on the pod, and may also be used by some tools that wish to recreate the pod (e.g. `podman generate systemd --new`). Optional. ShareParent determines if all containers in the pod will share the pod's cgroup as the cgroup parent
    #[serde(rename = "share_parent", skip_serializing_if = "Option::is_none")]
    pub share_parent: Option<bool>,
    /// SharedNamespaces instructs the pod to share a set of namespaces. Shared namespaces will be joined (by default) by every container which joins the pod. If not set and NoInfra is false, the pod will set a default set of namespaces to share. Conflicts with NoInfra=true. Optional.
    #[serde(rename = "shared_namespaces", skip_serializing_if = "Option::is_none")]
    pub shared_namespaces: Option<Vec<String>>,
    /// ShmSize is the size of the tmpfs to mount in at /dev/shm, in bytes. Conflicts with ShmSize if IpcNS is not private. Optional.
    #[serde(rename = "shm_size", skip_serializing_if = "Option::is_none")]
    pub shm_size: Option<i64>,
    /// ShmSizeSystemd is the size of systemd-specific tmpfs mounts specifically /run, /run/lock, /var/log/journal and /tmp. Optional
    #[serde(rename = "shm_size_systemd", skip_serializing_if = "Option::is_none")]
    pub shm_size_systemd: Option<i64>,
    /// Sysctl sets kernel parameters for the pod
    #[serde(rename = "sysctl", skip_serializing_if = "Option::is_none")]
    pub sysctl: Option<std::collections::HashMap<String, String>>,
    /// ThrottleReadBpsDevice contains the rate at which the devices in the pod can be read from/accessed
    #[serde(
        rename = "throttleReadBpsDevice",
        skip_serializing_if = "Option::is_none"
    )]
    pub throttle_read_bps_device:
        Option<std::collections::HashMap<String, models::LinuxThrottleDevice>>,
    #[serde(rename = "userns", skip_serializing_if = "Option::is_none")]
    pub userns: Option<Box<models::Namespace>>,
    #[serde(rename = "utsns", skip_serializing_if = "Option::is_none")]
    pub utsns: Option<Box<models::Namespace>>,
    /// Volumes are named volumes that will be added to the pod. These will supersede Image Volumes and VolumesFrom  volumes where there are conflicts. Optional.
    #[serde(rename = "volumes", skip_serializing_if = "Option::is_none")]
    pub volumes: Option<Vec<models::NamedVolume>>,
    /// VolumesFrom is a set of containers whose volumes will be added to this pod. The name or ID of the container must be provided, and may optionally be followed by a : and then one or more comma-separated options. Valid options are 'ro', 'rw', and 'z'. Options will be used for all volumes sourced from the container.
    #[serde(rename = "volumes_from", skip_serializing_if = "Option::is_none")]
    pub volumes_from: Option<Vec<String>>,
}

impl PodSpecGenerator {
    /// PodSpecGenerator describes options to create a pod
    pub fn new() -> PodSpecGenerator {
        PodSpecGenerator {
            networks: None,
            cgroup_parent: None,
            cni_networks: None,
            cpu_period: None,
            cpu_quota: None,
            dns_option: None,
            dns_search: None,
            dns_server: None,
            exit_policy: None,
            hostadd: None,
            hostname: None,
            idmappings: None,
            image_volumes: None,
            infra_command: None,
            infra_conmon_pid_file: None,
            infra_image: None,
            infra_name: None,
            ipcns: None,
            labels: None,
            mounts: None,
            name: None,
            netns: None,
            network_options: None,
            no_infra: None,
            no_manage_hosts: None,
            no_manage_resolv_conf: None,
            overlay_volumes: None,
            pidns: None,
            pod_create_command: None,
            pod_devices: None,
            portmappings: None,
            resource_limits: None,
            restart_policy: None,
            restart_tries: None,
            security_opt: None,
            service_container_id: None,
            share_parent: None,
            shared_namespaces: None,
            shm_size: None,
            shm_size_systemd: None,
            sysctl: None,
            throttle_read_bps_device: None,
            userns: None,
            utsns: None,
            volumes: None,
            volumes_from: None,
        }
    }
}
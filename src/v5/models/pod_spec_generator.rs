use serde::{Deserialize, Serialize};
#[derive(Default, Debug, Serialize, Deserialize)]
/// PodSpecGenerator describes options to create a pod
pub struct PodSpecGenerator {
    /// Map of networks names to ids the container should join to.
    /// You can request additional settings for each network, you can
    /// set network aliases, static ips, static mac address  and the
    /// network interface name for this container on the specific network.
    /// If the map is empty and the bridge network mode is set the container
    /// will be joined to the default network.
    #[serde(rename = "Networks")]
    pub networks:
        Option<std::collections::HashMap<String, Option<crate::v5::models::PerNetworkOptions>>>,
    /// CgroupParent is the parent for the Cgroup that the pod will create.
    /// This pod cgroup will, in turn, be the default cgroup parent for all
    /// containers in the pod.
    /// Optional.
    pub cgroup_parent: Option<String>,
    /// CNINetworks is a list of CNI networks to join the container to.
    /// If this list is empty, the default CNI network will be joined
    /// instead. If at least one entry is present, we will not join the
    /// default network (unless it is part of this list).
    /// Only available if NetNS is set to bridge.
    /// Optional.
    /// Deprecated: as of podman 4.0 use "Networks" instead.
    pub cni_networks: Option<Vec<String>>,
    /// CPU period of the cpuset, determined by --cpus
    pub cpu_period: Option<u64>,
    /// CPU quota of the cpuset, determined by --cpus
    pub cpu_quota: Option<i64>,
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
    /// ExitPolicy determines the pod's exit and stop behaviour.
    pub exit_policy: Option<String>,
    /// HostAdd is a set of hosts that will be added to the infra container's
    /// etc/hosts that will, by default, be shared with all containers in
    /// the pod.
    /// Conflicts with NoInfra=true and NoManageHosts.
    /// Optional.
    pub hostadd: Option<Vec<String>>,
    /// Hostname is the pod's hostname. If not set, the name of the pod will
    /// be used (if a name was not provided here, the name auto-generated for
    /// the pod will be used). This will be used by the infra container and
    /// all containers in the pod as long as the UTS namespace is shared.
    /// Optional.
    pub hostname: Option<String>,
    pub idmappings: Option<crate::v5::models::IdMappingOptions>,
    /// Image volumes bind-mount a container-image mount into the pod's infra container.
    /// Optional.
    pub image_volumes: Option<Vec<crate::v5::models::ImageVolume>>,
    /// InfraCommand sets the command that will be used to start the infra
    /// container.
    /// If not set, the default set in the Libpod configuration file will be
    /// used.
    /// Conflicts with NoInfra=true.
    /// Optional.
    pub infra_command: Option<Vec<String>>,
    /// InfraConmonPidFile is a custom path to store the infra container's
    /// conmon PID.
    pub infra_conmon_pid_file: Option<String>,
    /// InfraImage is the image that will be used for the infra container.
    /// If not set, the default set in the Libpod configuration file will be
    /// used.
    /// Conflicts with NoInfra=true.
    /// Optional.
    pub infra_image: Option<String>,
    /// InfraName is the name that will be used for the infra container.
    /// If not set, the default set in the Libpod configuration file will be
    /// used.
    /// Conflicts with NoInfra=true.
    /// Optional.
    pub infra_name: Option<String>,
    pub ipcns: Option<crate::v5::models::Namespace>,
    /// Labels are key-value pairs that are used to add metadata to pods.
    /// Optional.
    pub labels: Option<std::collections::HashMap<String, Option<String>>>,
    /// Mounts are mounts that will be added to the pod.
    /// These will supersede Image Volumes and VolumesFrom volumes where
    /// there are conflicts.
    /// Optional.
    pub mounts: Option<Vec<crate::v5::models::Mount>>,
    /// Name is the name of the pod.
    /// If not provided, a name will be generated when the pod is created.
    /// Optional.
    pub name: Option<String>,
    pub netns: Option<crate::v5::models::Namespace>,
    /// NetworkOptions are additional options for each network
    /// Optional.
    pub network_options: Option<std::collections::HashMap<String, Option<Vec<String>>>>,
    /// NoInfra tells the pod not to create an infra container. If this is
    /// done, many networking-related options will become unavailable.
    /// Conflicts with setting any options in PodNetworkConfig, and the
    /// InfraCommand and InfraImages in this struct.
    /// Optional.
    pub no_infra: Option<bool>,
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
    /// Overlay volumes are named volumes that will be added to the pod.
    /// Optional.
    pub overlay_volumes: Option<Vec<crate::v5::models::OverlayVolume>>,
    pub pidns: Option<crate::v5::models::Namespace>,
    pub pod_create_command: Option<Vec<String>>,
    /// Devices contains user specified Devices to be added to the Pod
    pub pod_devices: Option<Vec<String>>,
    /// PortMappings is a set of ports to map into the infra container.
    /// As, by default, containers share their network with the infra
    /// container, this will forward the ports to the entire pod.
    /// Only available if NetNS is set to Bridge, Slirp, or Pasta.
    /// Optional.
    pub portmappings: Option<Vec<crate::v5::models::PortMapping>>,
    pub resource_limits: Option<crate::v5::models::LinuxResources>,
    /// RestartPolicy is the pod's restart policy - an action which
    /// will be taken when one or all the containers in the pod exits.
    /// If not given, the default policy will be set to Always, which
    /// restarts the containers in the pod when they exit indefinitely.
    /// Optional.
    pub restart_policy: Option<String>,
    /// RestartRetries is the number of attempts that will be made to restart
    /// the container.
    /// Only available when RestartPolicy is set to "on-failure".
    /// Optional.
    pub restart_tries: Option<u64>,
    pub security_opt: Option<Vec<String>>,
    /// The ID of the pod's service container.
    #[serde(rename = "serviceContainerID")]
    pub service_container_id: Option<String>,
    /// PodCreateCommand is the command used to create this pod.
    /// This will be shown in the output of Inspect() on the pod, and may
    /// also be used by some tools that wish to recreate the pod
    /// (e.g. `podman generate systemd --new`).
    /// Optional.
    /// ShareParent determines if all containers in the pod will share the pod's cgroup as the cgroup parent
    pub share_parent: Option<bool>,
    /// SharedNamespaces instructs the pod to share a set of namespaces.
    /// Shared namespaces will be joined (by default) by every container
    /// which joins the pod.
    /// If not set and NoInfra is false, the pod will set a default set of
    /// namespaces to share.
    /// Conflicts with NoInfra=true.
    /// Optional.
    pub shared_namespaces: Option<Vec<String>>,
    /// ShmSize is the size of the tmpfs to mount in at /dev/shm, in bytes.
    /// Conflicts with ShmSize if IpcNS is not private.
    /// Optional.
    pub shm_size: Option<i64>,
    /// ShmSizeSystemd is the size of systemd-specific tmpfs mounts
    /// specifically /run, /run/lock, /var/log/journal and /tmp.
    /// Optional
    pub shm_size_systemd: Option<i64>,
    /// Sysctl sets kernel parameters for the pod
    pub sysctl: Option<std::collections::HashMap<String, Option<String>>>,
    /// ThrottleReadBpsDevice contains the rate at which the devices in the pod can be read from/accessed
    #[serde(rename = "throttleReadBpsDevice")]
    pub throttle_read_bps_device:
        Option<std::collections::HashMap<String, Option<crate::v5::models::LinuxThrottleDevice>>>,
    pub userns: Option<crate::v5::models::Namespace>,
    pub utsns: Option<crate::v5::models::Namespace>,
    /// Volumes are named volumes that will be added to the pod.
    /// These will supersede Image Volumes and VolumesFrom  volumes where
    /// there are conflicts.
    /// Optional.
    pub volumes: Option<Vec<crate::v5::models::NamedVolume>>,
    /// VolumesFrom is a set of containers whose volumes will be added to
    /// this pod. The name or ID of the container must be provided, and
    /// may optionally be followed by a : and then one or more
    /// comma-separated options. Valid options are 'ro', 'rw', and 'z'.
    /// Options will be used for all volumes sourced from the container.
    pub volumes_from: Option<Vec<String>>,
}

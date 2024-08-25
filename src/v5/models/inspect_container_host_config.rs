use serde::{Deserialize, Serialize};
#[derive(Default, Debug, Serialize, Deserialize)]
/// InspectContainerHostConfig holds information used when the container was
/// created.
/// It's very much a Docker-specific struct, retained (mostly) as-is for
/// compatibility. We fill individual fields as best as we can, inferring as much
/// as possible from the spec and container config.
/// Some things cannot be inferred. These will be populated by spec annotations
/// (if available).
/// nolint:revive,stylecheck // Field names are fixed for compatibility and cannot be changed.
pub struct InspectContainerHostConfig {
    /// Annotations are provided to the runtime when the container is
    /// started.
    #[serde(rename = "Annotations")]
    pub annotations: Option<std::collections::HashMap<String, Option<String>>>,
    /// AutoRemove is whether the container will be automatically removed on
    /// exiting.
    /// It is not handled directly within libpod and is stored in an
    /// annotation.
    #[serde(rename = "AutoRemove")]
    pub auto_remove: Option<bool>,
    /// Binds contains an array of user-added mounts.
    /// Both volume mounts and named volumes are included.
    /// Tmpfs mounts are NOT included.
    /// In 'docker inspect' this is separated into 'Binds' and 'Mounts' based
    /// on how a mount was added. We do not make this distinction and do not
    /// include a Mounts field in inspect.
    /// Format: <src>:<destination>[:<comma-separated options>]
    #[serde(rename = "Binds")]
    pub binds: Option<Vec<String>>,
    /// BlkioDeviceReadBps is an array of I/O throttle parameters for
    /// individual device nodes.
    /// This specifically sets read rate cap in bytes per second for device
    /// nodes.
    /// As with BlkioWeightDevice, we pull the path from /sys/dev, and we
    /// don't guarantee the path will be identical to the original (though
    /// the node will be).
    #[serde(rename = "BlkioDeviceReadBps")]
    pub blkio_device_read_bps: Option<Vec<crate::v5::models::InspectBlkioThrottleDevice>>,
    /// BlkioDeviceReadIOps is an array of I/O throttle parameters for
    /// individual device nodes.
    /// This specifically sets the read rate cap in iops per second for
    /// device nodes.
    /// As with BlkioWeightDevice, we pull the path from /sys/dev, and we
    /// don't guarantee the path will be identical to the original (though
    /// the node will be).
    #[serde(rename = "BlkioDeviceReadIOps")]
    pub blkio_device_read_i_ops: Option<Vec<crate::v5::models::InspectBlkioThrottleDevice>>,
    /// BlkioDeviceWriteBps is an array of I/O throttle parameters for
    /// individual device nodes.
    /// this specifically sets write rate cap in bytes per second for device
    /// nodes.
    /// as with BlkioWeightDevice, we pull the path from /sys/dev, and we
    /// don't guarantee the path will be identical to the original (though
    /// the node will be).
    #[serde(rename = "BlkioDeviceWriteBps")]
    pub blkio_device_write_bps: Option<Vec<crate::v5::models::InspectBlkioThrottleDevice>>,
    /// BlkioDeviceWriteIOps is an array of I/O throttle parameters for
    /// individual device nodes.
    /// This specifically sets the write rate cap in iops per second for
    /// device nodes.
    /// As with BlkioWeightDevice, we pull the path from /sys/dev, and we
    /// don't guarantee the path will be identical to the original (though
    /// the node will be).
    #[serde(rename = "BlkioDeviceWriteIOps")]
    pub blkio_device_write_i_ops: Option<Vec<crate::v5::models::InspectBlkioThrottleDevice>>,
    /// BlkioWeight indicates the I/O resources allocated to the container.
    /// It is a relative weight in the scheduler for assigning I/O time
    /// versus other Cgroups.
    #[serde(rename = "BlkioWeight")]
    pub blkio_weight: Option<u16>,
    /// BlkioWeightDevice is an array of I/O resource priorities for
    /// individual device nodes.
    /// Unfortunately, the spec only stores the device's Major/Minor numbers
    /// and not the path, which is used here.
    /// Fortunately, the kernel provides an interface for retrieving the path
    /// of a given node by major:minor at /sys/dev/. However, the exact path
    /// in use may not be what was used in the original CLI invocation -
    /// though it is guaranteed that the device node will be the same, and
    /// using the given path will be functionally identical.
    #[serde(rename = "BlkioWeightDevice")]
    pub blkio_weight_device: Option<Vec<crate::v5::models::InspectBlkioWeightDevice>>,
    /// CapAdd is a list of capabilities added to the container.
    /// It is not directly stored by Libpod, and instead computed from the
    /// capabilities listed in the container's spec, compared against a set
    /// of default capabilities.
    #[serde(rename = "CapAdd")]
    pub cap_add: Option<Vec<String>>,
    /// CapDrop is a list of capabilities removed from the container.
    /// It is not directly stored by libpod, and instead computed from the
    /// capabilities listed in the container's spec, compared against a set
    /// of default capabilities.
    #[serde(rename = "CapDrop")]
    pub cap_drop: Option<Vec<String>>,
    /// Cgroup contains the container's cgroup. It is presently not
    /// populated.
    /// TODO.
    #[serde(rename = "Cgroup")]
    pub cgroup: Option<String>,
    /// CgroupConf is the configuration for cgroup v2.
    #[serde(rename = "CgroupConf")]
    pub cgroup_conf: Option<std::collections::HashMap<String, Option<String>>>,
    /// CgroupManager is the cgroup manager used by the container.
    /// At present, allowed values are either "cgroupfs" or "systemd".
    #[serde(rename = "CgroupManager")]
    pub cgroup_manager: Option<String>,
    /// CgroupMode is the configuration of the container's cgroup namespace.
    /// Populated as follows:
    /// private - a cgroup namespace has been created
    /// host - No cgroup namespace created
    /// container:<id> - Using another container's cgroup namespace
    /// ns:<path> - A path to a cgroup namespace has been specified
    #[serde(rename = "CgroupMode")]
    pub cgroup_mode: Option<String>,
    /// CgroupParent is the Cgroup parent of the container.
    /// Only set if not default.
    #[serde(rename = "CgroupParent")]
    pub cgroup_parent: Option<String>,
    /// Cgroups contains the container's Cgroup mode.
    /// Allowed values are "default" (container is creating Cgroups) and
    /// "disabled" (container is not creating Cgroups).
    /// This is Libpod-specific and not included in `docker inspect`.
    #[serde(rename = "Cgroups")]
    pub cgroups: Option<String>,
    /// ConsoleSize is an array of 2 integers showing the size of the
    /// container's console.
    /// It is only set if the container is creating a terminal.
    /// TODO.
    #[serde(rename = "ConsoleSize")]
    pub console_size: Option<Vec<u64>>,
    /// ContainerIDFile is a file created during container creation to hold
    /// the ID of the created container.
    /// This is not handled within libpod and is stored in an annotation.
    #[serde(rename = "ContainerIDFile")]
    pub container_id_file: Option<String>,
    /// CpuCount is Windows-only and not presently implemented.
    #[serde(rename = "CpuCount")]
    pub cpu_count: Option<u64>,
    /// CpuPercent is Windows-only and not presently implemented.
    #[serde(rename = "CpuPercent")]
    pub cpu_percent: Option<u64>,
    /// CpuPeriod is the length of a CPU period in microseconds.
    /// It relates directly to CpuQuota.
    #[serde(rename = "CpuPeriod")]
    pub cpu_period: Option<u64>,
    /// CpuPeriod is the amount of time (in microseconds) that a container
    /// can use the CPU in every CpuPeriod.
    #[serde(rename = "CpuQuota")]
    pub cpu_quota: Option<i64>,
    /// CpuRealtimePeriod is the length of time (in microseconds) of the CPU
    /// realtime period. If set to 0, no time will be allocated to realtime
    /// tasks.
    #[serde(rename = "CpuRealtimePeriod")]
    pub cpu_realtime_period: Option<u64>,
    /// CpuRealtimeRuntime is the length of time (in microseconds) allocated
    /// for realtime tasks within every CpuRealtimePeriod.
    #[serde(rename = "CpuRealtimeRuntime")]
    pub cpu_realtime_runtime: Option<i64>,
    /// CpuShares indicates the CPU resources allocated to the container.
    /// It is a relative weight in the scheduler for assigning CPU time
    /// versus other Cgroups.
    #[serde(rename = "CpuShares")]
    pub cpu_shares: Option<u64>,
    /// CpusetCpus is the set of CPUs that the container will execute on.
    /// Formatted as `0-3` or `0,2`. Default (if unset) is all CPUs.
    #[serde(rename = "CpusetCpus")]
    pub cpuset_cpus: Option<String>,
    /// CpusetMems is the set of memory nodes the container will use.
    /// Formatted as `0-3` or `0,2`. Default (if unset) is all memory nodes.
    #[serde(rename = "CpusetMems")]
    pub cpuset_mems: Option<String>,
    /// Devices is a list of device nodes that will be added to the
    /// container.
    /// These are stored in the OCI spec only as type, major, minor while we
    /// display the host path. We convert this with /sys/dev, but we cannot
    /// guarantee that the host path will be identical - only that the actual
    /// device will be.
    #[serde(rename = "Devices")]
    pub devices: Option<Vec<crate::v5::models::InspectDevice>>,
    /// DiskQuota is the maximum amount of disk space the container may use
    /// (in bytes).
    /// Presently not populated.
    /// TODO.
    #[serde(rename = "DiskQuota")]
    pub disk_quota: Option<u64>,
    /// Dns is a list of DNS nameservers that will be added to the
    /// container's resolv.conf
    #[serde(rename = "Dns")]
    pub dns: Option<Vec<String>>,
    /// DnsOptions is a list of DNS options that will be set in the
    /// container's resolv.conf
    #[serde(rename = "DnsOptions")]
    pub dns_options: Option<Vec<String>>,
    /// DnsSearch is a list of DNS search domains that will be set in the
    /// container's resolv.conf
    #[serde(rename = "DnsSearch")]
    pub dns_search: Option<Vec<String>>,
    /// ExtraHosts contains hosts that will be added to the container's
    /// etc/hosts.
    #[serde(rename = "ExtraHosts")]
    pub extra_hosts: Option<Vec<String>>,
    /// GroupAdd contains groups that the user inside the container will be
    /// added to.
    #[serde(rename = "GroupAdd")]
    pub group_add: Option<Vec<String>>,
    #[serde(rename = "IDMappings")]
    pub id_mappings: Option<crate::v5::models::InspectIdMappings>,
    /// IOMaximumBandwidth is Windows-only and not presently implemented.
    #[serde(rename = "IOMaximumBandwidth")]
    pub io_maximum_bandwidth: Option<u64>,
    /// IOMaximumIOps is Windows-only and not presently implemented.
    #[serde(rename = "IOMaximumIOps")]
    pub io_maximum_i_ops: Option<u64>,
    /// Init indicates whether the container has an init mounted into it.
    #[serde(rename = "Init")]
    pub init: Option<bool>,
    /// IntelRdtClosID defines the Intel RDT CAT Class Of Service (COS) that
    /// all processes of the container should run in.
    #[serde(rename = "IntelRdtClosID")]
    pub intel_rdt_clos_id: Option<String>,
    /// IpcMode represents the configuration of the container's IPC
    /// namespace.
    /// Populated as follows:
    /// "" (empty string) - Default, an IPC namespace will be created
    /// host - No IPC namespace created
    /// container:<id> - Using another container's IPC namespace
    /// ns:<path> - A path to an IPC namespace has been specified
    #[serde(rename = "IpcMode")]
    pub ipc_mode: Option<String>,
    /// Isolation is presently unused and provided solely for Docker
    /// compatibility.
    #[serde(rename = "Isolation")]
    pub isolation: Option<String>,
    /// KernelMemory is the maximum amount of memory the kernel will devote
    /// to the container.
    #[serde(rename = "KernelMemory")]
    pub kernel_memory: Option<i64>,
    /// Links is unused, and provided purely for Docker compatibility.
    #[serde(rename = "Links")]
    pub links: Option<Vec<String>>,
    #[serde(rename = "LogConfig")]
    pub log_config: Option<crate::v5::models::InspectLogConfig>,
    /// Memory indicates the memory resources allocated to the container.
    /// This is the limit (in bytes) of RAM the container may use.
    #[serde(rename = "Memory")]
    pub memory: Option<i64>,
    /// MemoryReservation is the reservation (soft limit) of memory available
    /// to the container. Soft limits are warnings only and can be exceeded.
    #[serde(rename = "MemoryReservation")]
    pub memory_reservation: Option<i64>,
    /// MemorySwap is the total limit for all memory available to the
    /// container, including swap. 0 indicates that there is no limit to the
    /// amount of memory available.
    #[serde(rename = "MemorySwap")]
    pub memory_swap: Option<i64>,
    /// MemorySwappiness is the willingness of the kernel to page container
    /// memory to swap. It is an integer from 0 to 100, with low numbers
    /// being more likely to be put into swap.
    /// 1, the default, will not set swappiness and use the system defaults.
    #[serde(rename = "MemorySwappiness")]
    pub memory_swappiness: Option<i64>,
    /// NanoCpus indicates number of CPUs allocated to the container.
    /// It is an integer where one full CPU is indicated by 1000000000 (one
    /// billion).
    /// Thus, 2.5 CPUs (fractional portions of CPUs are allowed) would be
    /// 2500000000 (2.5 billion).
    /// In 'docker inspect' this is set exclusively of two further options in
    /// the output (CpuPeriod and CpuQuota) which are both used to implement
    /// this functionality.
    /// We can't distinguish here, so if CpuQuota is set to the default of
    /// 100000, we will set both CpuQuota, CpuPeriod, and NanoCpus. If
    /// CpuQuota is not the default, we will not set NanoCpus.
    #[serde(rename = "NanoCpus")]
    pub nano_cpus: Option<i64>,
    /// NetworkMode is the configuration of the container's network
    /// namespace.
    /// Populated as follows:
    /// default - A network namespace is being created and configured via CNI
    /// none - A network namespace is being created, not configured via CNI
    /// host - No network namespace created
    /// container:<id> - Using another container's network namespace
    /// ns:<path> - A path to a network namespace has been specified
    #[serde(rename = "NetworkMode")]
    pub network_mode: Option<String>,
    /// OomKillDisable indicates whether the kernel OOM killer is disabled
    /// for the container.
    #[serde(rename = "OomKillDisable")]
    pub oom_kill_disable: Option<bool>,
    /// OOMScoreAdj is an adjustment that will be made to the container's OOM
    /// score.
    #[serde(rename = "OomScoreAdj")]
    pub oom_score_adj: Option<i64>,
    /// PidMode represents the configuration of the container's PID
    /// namespace.
    /// Populated as follows:
    /// "" (empty string) - Default, a PID namespace will be created
    /// host - No PID namespace created
    /// container:<id> - Using another container's PID namespace
    /// ns:<path> - A path to a PID namespace has been specified
    #[serde(rename = "PidMode")]
    pub pid_mode: Option<String>,
    /// PidsLimit is the maximum number of PIDs that may be created within
    /// the container. 0, the default, indicates no limit.
    #[serde(rename = "PidsLimit")]
    pub pids_limit: Option<i64>,
    /// PortBindings contains the container's port bindings.
    /// It is formatted as map[string][]InspectHostPort.
    /// The string key here is formatted as <integer port number>/<protocol>
    /// and represents the container port. A single container port may be
    /// bound to multiple host ports (on different IPs).
    #[serde(rename = "PortBindings")]
    pub port_bindings:
        Option<std::collections::HashMap<String, Option<Vec<crate::v5::models::InspectHostPort>>>>,
    /// Privileged indicates whether the container is running with elevated
    /// privileges.
    /// This has a very specific meaning in the Docker sense, so it's very
    /// difficult to decode from the spec and config, and so is stored as an
    /// annotation.
    #[serde(rename = "Privileged")]
    pub privileged: Option<bool>,
    /// PublishAllPorts indicates whether image ports are being published.
    /// This is not directly stored in libpod and is saved as an annotation.
    #[serde(rename = "PublishAllPorts")]
    pub publish_all_ports: Option<bool>,
    /// ReadonlyRootfs is whether the container will be mounted read-only.
    #[serde(rename = "ReadonlyRootfs")]
    pub readonly_rootfs: Option<bool>,
    #[serde(rename = "RestartPolicy")]
    pub restart_policy: Option<crate::v5::models::InspectRestartPolicy>,
    /// Runtime is provided purely for Docker compatibility.
    /// It is set unconditionally to "oci" as Podman does not presently
    /// support non-OCI runtimes.
    #[serde(rename = "Runtime")]
    pub runtime: Option<String>,
    /// SecurityOpt is a list of security-related options that are set in the
    /// container.
    #[serde(rename = "SecurityOpt")]
    pub security_opt: Option<Vec<String>>,
    #[serde(rename = "ShmSize")]
    pub shm_size: Option<i64>,
    /// Tmpfs is a list of tmpfs filesystems that will be mounted into the
    /// container.
    /// It is a map of destination path to options for the mount.
    #[serde(rename = "Tmpfs")]
    pub tmpfs: Option<std::collections::HashMap<String, Option<String>>>,
    /// UTSMode represents the configuration of the container's UID
    /// namespace.
    /// Populated as follows:
    /// "" (empty string) - Default, a UTS namespace will be created
    /// host - no UTS namespace created
    /// container:<id> - Using another container's UTS namespace
    /// ns:<path> - A path to a UTS namespace has been specified
    #[serde(rename = "UTSMode")]
    pub uts_mode: Option<String>,
    /// Ulimits is a set of ulimits that will be set within the container.
    #[serde(rename = "Ulimits")]
    pub ulimits: Option<Vec<crate::v5::models::InspectUlimit>>,
    /// UsernsMode represents the configuration of the container's user
    /// namespace.
    /// When running rootless, a user namespace is created outside of libpod
    /// to allow some privileged operations. This will not be reflected here.
    /// Populated as follows:
    /// "" (empty string) - No user namespace will be created
    /// private - The container will be run in a user namespace
    /// container:<id> - Using another container's user namespace
    /// ns:<path> - A path to a user namespace has been specified
    /// TODO Rootless has an additional 'keep-id' option, presently not
    /// reflected here.
    #[serde(rename = "UsernsMode")]
    pub userns_mode: Option<String>,
    /// VolumeDriver is presently unused and is retained for Docker
    /// compatibility.
    #[serde(rename = "VolumeDriver")]
    pub volume_driver: Option<String>,
    /// VolumesFrom is a list of containers which this container uses volumes
    /// from. This is not handled directly within libpod and is stored in an
    /// annotation.
    /// It is formatted as an array of container names and IDs.
    #[serde(rename = "VolumesFrom")]
    pub volumes_from: Option<Vec<String>>,
}

use serde::{Deserialize, Serialize};
#[derive(Default, Debug, Serialize, Deserialize)]
/// HostConfig the non-portable Config structure of a container.
/// Here, "non-portable" means "dependent of the host we are running on".
/// Portable information *should* appear in Config.
pub struct HostConfig {
    #[serde(rename = "Annotations")]
    pub annotations: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "AutoRemove")]
    pub auto_remove: Option<bool>,
    /// Applicable to all platforms
    #[serde(rename = "Binds")]
    pub binds: Option<Vec<String>>,
    #[serde(rename = "BlkioDeviceReadBps")]
    pub blkio_device_read_bps: Option<Vec<super::super::models::ThrottleDevice>>,
    #[serde(rename = "BlkioDeviceReadIOps")]
    pub blkio_device_read_i_ops: Option<Vec<super::super::models::ThrottleDevice>>,
    #[serde(rename = "BlkioDeviceWriteBps")]
    pub blkio_device_write_bps: Option<Vec<super::super::models::ThrottleDevice>>,
    #[serde(rename = "BlkioDeviceWriteIOps")]
    pub blkio_device_write_i_ops: Option<Vec<super::super::models::ThrottleDevice>>,
    #[serde(rename = "BlkioWeight")]
    pub blkio_weight: Option<u16>,
    #[serde(rename = "BlkioWeightDevice")]
    pub blkio_weight_device: Option<Vec<super::super::models::WeightDevice>>,
    #[serde(rename = "CapAdd")]
    pub cap_add: Option<Vec<String>>,
    #[serde(rename = "CapDrop")]
    pub cap_drop: Option<Vec<String>>,
    #[serde(rename = "Cgroup")]
    pub cgroup: Option<String>,
    /// Applicable to UNIX platforms
    #[serde(rename = "CgroupParent")]
    pub cgroup_parent: Option<String>,
    #[serde(rename = "CgroupnsMode")]
    pub cgroupns_mode: Option<String>,
    #[serde(rename = "ConsoleSize")]
    pub console_size: Option<Vec<u64>>,
    #[serde(rename = "ContainerIDFile")]
    pub container_id_file: Option<String>,
    /// Applicable to Windows
    #[serde(rename = "CpuCount")]
    pub cpu_count: Option<i64>,
    #[serde(rename = "CpuPercent")]
    pub cpu_percent: Option<i64>,
    #[serde(rename = "CpuPeriod")]
    pub cpu_period: Option<i64>,
    #[serde(rename = "CpuQuota")]
    pub cpu_quota: Option<i64>,
    #[serde(rename = "CpuRealtimePeriod")]
    pub cpu_realtime_period: Option<i64>,
    #[serde(rename = "CpuRealtimeRuntime")]
    pub cpu_realtime_runtime: Option<i64>,
    /// Applicable to all platforms
    #[serde(rename = "CpuShares")]
    pub cpu_shares: Option<i64>,
    #[serde(rename = "CpusetCpus")]
    pub cpuset_cpus: Option<String>,
    #[serde(rename = "CpusetMems")]
    pub cpuset_mems: Option<String>,
    #[serde(rename = "DeviceCgroupRules")]
    pub device_cgroup_rules: Option<Vec<String>>,
    #[serde(rename = "DeviceRequests")]
    pub device_requests: Option<Vec<super::super::models::DeviceRequest>>,
    #[serde(rename = "Devices")]
    pub devices: Option<Vec<super::super::models::DeviceMapping>>,
    #[serde(rename = "Dns")]
    pub dns: Option<Vec<String>>,
    #[serde(rename = "DnsOptions")]
    pub dns_options: Option<Vec<String>>,
    #[serde(rename = "DnsSearch")]
    pub dns_search: Option<Vec<String>>,
    #[serde(rename = "ExtraHosts")]
    pub extra_hosts: Option<Vec<String>>,
    #[serde(rename = "GroupAdd")]
    pub group_add: Option<Vec<String>>,
    #[serde(rename = "IOMaximumBandwidth")]
    pub io_maximum_bandwidth: Option<u64>,
    #[serde(rename = "IOMaximumIOps")]
    pub io_maximum_i_ops: Option<u64>,
    /// Run a custom init inside the container, if null, use the daemon's configured settings
    #[serde(rename = "Init")]
    pub init: Option<bool>,
    #[serde(rename = "IpcMode")]
    pub ipc_mode: Option<String>,
    #[serde(rename = "Isolation")]
    pub isolation: Option<String>,
    /// KernelMemory specifies the kernel memory limit (in bytes) for the container.
    /// Deprecated: kernel 5.4 deprecated kmem.limit_in_bytes.
    #[serde(rename = "KernelMemory")]
    pub kernel_memory: Option<i64>,
    #[serde(rename = "KernelMemoryTCP")]
    pub kernel_memory_tcp: Option<i64>,
    #[serde(rename = "Links")]
    pub links: Option<Vec<String>>,
    #[serde(rename = "LogConfig")]
    pub log_config: Option<super::super::models::LogConfig>,
    /// MaskedPaths is the list of paths to be masked inside the container (this overrides the default set of paths)
    #[serde(rename = "MaskedPaths")]
    pub masked_paths: Option<Vec<String>>,
    #[serde(rename = "Memory")]
    pub memory: Option<i64>,
    #[serde(rename = "MemoryReservation")]
    pub memory_reservation: Option<i64>,
    #[serde(rename = "MemorySwap")]
    pub memory_swap: Option<i64>,
    #[serde(rename = "MemorySwappiness")]
    pub memory_swappiness: Option<i64>,
    /// Mounts specs used by the container
    #[serde(rename = "Mounts")]
    pub mounts: Option<Vec<super::super::models::Mount>>,
    #[serde(rename = "NanoCpus")]
    pub nano_cpus: Option<i64>,
    #[serde(rename = "NetworkMode")]
    pub network_mode: Option<String>,
    #[serde(rename = "OomKillDisable")]
    pub oom_kill_disable: Option<bool>,
    #[serde(rename = "OomScoreAdj")]
    pub oom_score_adj: Option<i64>,
    #[serde(rename = "PidMode")]
    pub pid_mode: Option<String>,
    #[serde(rename = "PidsLimit")]
    pub pids_limit: Option<i64>,
    #[serde(rename = "PortBindings")]
    pub port_bindings:
        Option<std::collections::HashMap<String, Vec<super::super::models::PortBinding>>>,
    #[serde(rename = "Privileged")]
    pub privileged: Option<bool>,
    #[serde(rename = "PublishAllPorts")]
    pub publish_all_ports: Option<bool>,
    /// ReadonlyPaths is the list of paths to be set as read-only inside the container (this overrides the default set of paths)
    #[serde(rename = "ReadonlyPaths")]
    pub readonly_paths: Option<Vec<String>>,
    #[serde(rename = "ReadonlyRootfs")]
    pub readonly_rootfs: Option<bool>,
    #[serde(rename = "RestartPolicy")]
    pub restart_policy: Option<super::super::models::RestartPolicy>,
    #[serde(rename = "Runtime")]
    pub runtime: Option<String>,
    #[serde(rename = "SecurityOpt")]
    pub security_opt: Option<Vec<String>>,
    #[serde(rename = "ShmSize")]
    pub shm_size: Option<i64>,
    #[serde(rename = "StorageOpt")]
    pub storage_opt: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "Sysctls")]
    pub sysctls: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "Tmpfs")]
    pub tmpfs: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "UTSMode")]
    pub uts_mode: Option<String>,
    #[serde(rename = "Ulimits")]
    pub ulimits: Option<Vec<super::super::models::Ulimit>>,
    #[serde(rename = "UsernsMode")]
    pub userns_mode: Option<String>,
    #[serde(rename = "VolumeDriver")]
    pub volume_driver: Option<String>,
    #[serde(rename = "VolumesFrom")]
    pub volumes_from: Option<Vec<String>>,
}

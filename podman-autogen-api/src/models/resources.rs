use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Serialize, Deserialize)]
/// Resources contains container's resources (cgroups config, ulimits...)
pub struct Resources {
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

    /// Applicable to UNIX platforms
    #[serde(rename = "CgroupParent")]
    pub cgroup_parent: Option<String>,

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

    #[serde(rename = "IOMaximumBandwidth")]
    pub io_maximum_bandwidth: Option<u64>,

    #[serde(rename = "IOMaximumIOps")]
    pub io_maximum_i_ops: Option<u64>,

    /// KernelMemory specifies the kernel memory limit (in bytes) for the container.
    /// Deprecated: kernel 5.4 deprecated kmem.limit_in_bytes.
    #[serde(rename = "KernelMemory")]
    pub kernel_memory: Option<i64>,

    #[serde(rename = "KernelMemoryTCP")]
    pub kernel_memory_tcp: Option<i64>,

    #[serde(rename = "Memory")]
    pub memory: Option<i64>,

    #[serde(rename = "MemoryReservation")]
    pub memory_reservation: Option<i64>,

    #[serde(rename = "MemorySwap")]
    pub memory_swap: Option<i64>,

    #[serde(rename = "MemorySwappiness")]
    pub memory_swappiness: Option<i64>,

    #[serde(rename = "NanoCpus")]
    pub nano_cpus: Option<i64>,

    #[serde(rename = "OomKillDisable")]
    pub oom_kill_disable: Option<bool>,

    #[serde(rename = "PidsLimit")]
    pub pids_limit: Option<i64>,

    #[serde(rename = "Ulimits")]
    pub ulimits: Option<Vec<super::super::models::Ulimit>>,
}

use serde::{Deserialize, Serialize};
#[derive(Default, Debug, Serialize, Deserialize)]
/// ContainerResourceConfig contains information on container resource limits.
pub struct ContainerResourceConfig {
    #[serde(rename = "intelRdt")]
    pub intel_rdt: Option<super::super::models::LinuxIntelRdt>,
    /// OOMScoreAdj adjusts the score used by the OOM killer to determine
    /// processes to kill for the container's process.
    /// Optional.
    pub oom_score_adj: Option<i64>,
    /// Rlimits are POSIX rlimits to apply to the container.
    /// Optional.
    pub r_limits: Option<Vec<super::super::models::PosixRlimit>>,
    pub resource_limits: Option<super::super::models::LinuxResources>,
    /// IO read rate limit per cgroup per device, bytes per second
    #[serde(rename = "throttleReadBpsDevice")]
    pub throttle_read_bps_device:
        Option<std::collections::HashMap<String, super::super::models::LinuxThrottleDevice>>,
    /// IO read rate limit per cgroup per device, IO per second
    #[serde(rename = "throttleReadIOPSDevice")]
    pub throttle_read_iops_device:
        Option<std::collections::HashMap<String, super::super::models::LinuxThrottleDevice>>,
    /// IO write rate limit per cgroup per device, bytes per second
    #[serde(rename = "throttleWriteBpsDevice")]
    pub throttle_write_bps_device:
        Option<std::collections::HashMap<String, super::super::models::LinuxThrottleDevice>>,
    /// IO write rate limit per cgroup per device, IO per second
    #[serde(rename = "throttleWriteIOPSDevice")]
    pub throttle_write_iops_device:
        Option<std::collections::HashMap<String, super::super::models::LinuxThrottleDevice>>,
    /// CgroupConf are key-value options passed into the container runtime
    /// that are used to configure cgroup v2.
    /// Optional.
    pub unified: Option<std::collections::HashMap<String, String>>,
    /// Weight per cgroup per device, can override BlkioWeight
    #[serde(rename = "weightDevice")]
    pub weight_device:
        Option<std::collections::HashMap<String, super::super::models::LinuxWeightDevice>>,
}

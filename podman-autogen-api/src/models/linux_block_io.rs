use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Serialize, Deserialize)]
/// LinuxBlockIO for Linux cgroup 'blkio' resource management
pub struct LinuxBlockIo {
    /// Specifies tasks' weight in the given cgroup while competing with the cgroup's child cgroups, CFQ scheduler only
    #[serde(rename = "leafWeight")]
    pub leaf_weight: Option<u16>,

    /// IO read rate limit per cgroup per device, bytes per second
    #[serde(rename = "throttleReadBpsDevice")]
    pub throttle_read_bps_device: Option<Vec<super::super::models::LinuxThrottleDevice>>,

    /// IO read rate limit per cgroup per device, IO per second
    #[serde(rename = "throttleReadIOPSDevice")]
    pub throttle_read_iops_device: Option<Vec<super::super::models::LinuxThrottleDevice>>,

    /// IO write rate limit per cgroup per device, bytes per second
    #[serde(rename = "throttleWriteBpsDevice")]
    pub throttle_write_bps_device: Option<Vec<super::super::models::LinuxThrottleDevice>>,

    /// IO write rate limit per cgroup per device, IO per second
    #[serde(rename = "throttleWriteIOPSDevice")]
    pub throttle_write_iops_device: Option<Vec<super::super::models::LinuxThrottleDevice>>,

    /// Specifies per cgroup weight
    pub weight: Option<u16>,

    /// Weight per cgroup per device, can override BlkioWeight
    #[serde(rename = "weightDevice")]
    pub weight_device: Option<Vec<super::super::models::LinuxWeightDevice>>,
}

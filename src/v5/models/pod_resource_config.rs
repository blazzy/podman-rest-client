use serde::{Deserialize, Serialize};
#[derive(Default, Debug, Serialize, Deserialize)]
pub struct PodResourceConfig {
    /// CPU period of the cpuset, determined by --cpus
    pub cpu_period: Option<u64>,
    /// CPU quota of the cpuset, determined by --cpus
    pub cpu_quota: Option<i64>,
    pub resource_limits: Option<super::super::models::LinuxResources>,
    /// ThrottleReadBpsDevice contains the rate at which the devices in the pod can be read from/accessed
    #[serde(rename = "throttleReadBpsDevice")]
    pub throttle_read_bps_device:
        Option<std::collections::HashMap<String, super::super::models::LinuxThrottleDevice>>,
}

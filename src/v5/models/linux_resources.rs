use serde::{Deserialize, Serialize};
#[derive(Default, Debug, Serialize, Deserialize)]
/// LinuxResources has container runtime resource constraints
pub struct LinuxResources {
    #[serde(rename = "blockIO")]
    pub block_io: Option<crate::v5::models::LinuxBlockIo>,
    pub cpu: Option<crate::v5::models::LinuxCpu>,
    /// Devices configures the device allowlist.
    pub devices: Option<Vec<crate::v5::models::LinuxDeviceCgroup>>,
    /// Hugetlb limits (in bytes). Default to reservation limits if supported.
    #[serde(rename = "hugepageLimits")]
    pub hugepage_limits: Option<Vec<crate::v5::models::LinuxHugepageLimit>>,
    pub memory: Option<crate::v5::models::LinuxMemory>,
    pub network: Option<crate::v5::models::LinuxNetwork>,
    pub pids: Option<crate::v5::models::LinuxPids>,
    /// Rdma resource restriction configuration.
    /// Limits are a set of key value pairs that define RDMA resource limits,
    /// where the key is device name and value is resource limits.
    pub rdma: Option<std::collections::HashMap<String, Option<crate::v5::models::LinuxRdma>>>,
    /// Unified resources.
    pub unified: Option<std::collections::HashMap<String, Option<String>>>,
}

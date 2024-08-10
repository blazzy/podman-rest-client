use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Serialize, Deserialize)]
/// LinuxResources has container runtime resource constraints
pub struct LinuxResources {
    #[serde(rename = "blockIO")]
    pub block_io: Option<super::super::models::LinuxBlockIo>,

    pub cpu: Option<super::super::models::LinuxCpu>,

    /// Devices configures the device allowlist.
    pub devices: Option<Vec<super::super::models::LinuxDeviceCgroup>>,

    /// Hugetlb limits (in bytes). Default to reservation limits if supported.
    #[serde(rename = "hugepageLimits")]
    pub hugepage_limits: Option<Vec<super::super::models::LinuxHugepageLimit>>,

    pub memory: Option<super::super::models::LinuxMemory>,

    pub network: Option<super::super::models::LinuxNetwork>,

    pub pids: Option<super::super::models::LinuxPids>,

    /// Rdma resource restriction configuration.
    /// Limits are a set of key value pairs that define RDMA resource limits,
    /// where the key is device name and value is resource limits.
    pub rdma: Option<std::collections::HashMap<String, super::super::models::LinuxRdma>>,

    /// Unified resources.
    pub unified: Option<std::collections::HashMap<String, String>>,
}

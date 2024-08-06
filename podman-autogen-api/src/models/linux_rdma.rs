use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Serialize, Deserialize)]
/// LinuxRdma for Linux cgroup 'rdma' resource management (Linux 4.11)
pub struct LinuxRdma {
    /// Maximum number of HCA handles that can be opened. Default is "no limit".
    #[serde(rename = "hcaHandles")]
    pub hca_handles: Option<u32>,

    /// Maximum number of HCA objects that can be created. Default is "no limit".
    #[serde(rename = "hcaObjects")]
    pub hca_objects: Option<u32>,
}

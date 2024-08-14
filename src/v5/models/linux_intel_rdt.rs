use serde::{Deserialize, Serialize};
#[derive(Default, Debug, Serialize, Deserialize)]
/// LinuxIntelRdt has container runtime resource constraints for Intel RDT CAT and MBA
/// features and flags enabling Intel RDT CMT and MBM features.
/// Intel RDT features are available in Linux 4.14 and newer kernel versions.
pub struct LinuxIntelRdt {
    /// The identity for RDT Class of Service
    #[serde(rename = "closID")]
    pub clos_id: Option<String>,
    /// EnableCMT is the flag to indicate if the Intel RDT CMT is enabled. CMT (Cache Monitoring Technology) supports monitoring of
    /// the last-level cache (LLC) occupancy for the container.
    #[serde(rename = "enableCMT")]
    pub enable_cmt: Option<bool>,
    /// EnableMBM is the flag to indicate if the Intel RDT MBM is enabled. MBM (Memory Bandwidth Monitoring) supports monitoring of
    /// total and local memory bandwidth for the container.
    #[serde(rename = "enableMBM")]
    pub enable_mbm: Option<bool>,
    /// The schema for L3 cache id and capacity bitmask (CBM)
    /// Format: "L3:<cache_id0>=<cbm0>;<cache_id1>=<cbm1>;..."
    #[serde(rename = "l3CacheSchema")]
    pub l3_cache_schema: Option<String>,
    /// The schema of memory bandwidth per L3 cache id
    /// Format: "MB:<cache_id0>=bandwidth0;<cache_id1>=bandwidth1;..."
    /// The unit of memory bandwidth is specified in "percentages" by
    /// default, and in "MBps" if MBA Software Controller is enabled.
    #[serde(rename = "memBwSchema")]
    pub mem_bw_schema: Option<String>,
}

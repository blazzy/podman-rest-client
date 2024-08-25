use serde::{Deserialize, Serialize};
#[derive(Default, Debug, Serialize, Deserialize)]
/// ContainerStats contains the statistics information for a running container
pub struct ContainerStats {
    #[serde(rename = "AvgCPU")]
    pub avg_cpu: Option<f64>,
    #[serde(rename = "BlockInput")]
    pub block_input: Option<u64>,
    #[serde(rename = "BlockOutput")]
    pub block_output: Option<u64>,
    #[serde(rename = "CPU")]
    pub cpu: Option<f64>,
    #[serde(rename = "CPUNano")]
    pub cpu_nano: Option<u64>,
    #[serde(rename = "CPUSystemNano")]
    pub cpu_system_nano: Option<u64>,
    #[serde(rename = "ContainerID")]
    pub container_id: Option<String>,
    #[serde(rename = "Duration")]
    pub duration: Option<u64>,
    #[serde(rename = "MemLimit")]
    pub mem_limit: Option<u64>,
    #[serde(rename = "MemPerc")]
    pub mem_perc: Option<f64>,
    #[serde(rename = "MemUsage")]
    pub mem_usage: Option<u64>,
    #[serde(rename = "Name")]
    pub name: Option<String>,
    /// Map of interface name to network statistics for that interface.
    #[serde(rename = "Network")]
    pub network:
        Option<std::collections::HashMap<String, Option<crate::v5::models::ContainerNetworkStats>>>,
    #[serde(rename = "PIDs")]
    pub pi_ds: Option<u64>,
    #[serde(rename = "PerCPU")]
    pub per_cpu: Option<Vec<u64>>,
    #[serde(rename = "SystemNano")]
    pub system_nano: Option<u64>,
    #[serde(rename = "UpTime")]
    pub up_time: Option<i64>,
}

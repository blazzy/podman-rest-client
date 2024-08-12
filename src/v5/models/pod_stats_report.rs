use serde::{Deserialize, Serialize};
#[derive(Default, Debug, Serialize, Deserialize)]
/// PodStatsReport includes pod-resource statistics data.
pub struct PodStatsReport {
    /// Humanized disk usage read + write
    #[serde(rename = "BlockIO")]
    pub block_io: Option<String>,
    /// Container ID
    #[serde(rename = "CID")]
    pub cid: Option<String>,
    /// Percentage of CPU utilized by pod
    #[serde(rename = "CPU")]
    pub cpu: Option<String>,
    /// Percentage of Memory utilized by pod
    #[serde(rename = "Mem")]
    pub mem: Option<String>,
    /// Humanized Memory usage and maximum
    #[serde(rename = "MemUsage")]
    pub mem_usage: Option<String>,
    /// Memory usage and maximum in bytes
    #[serde(rename = "MemUsageBytes")]
    pub mem_usage_bytes: Option<String>,
    /// Pod Name
    #[serde(rename = "Name")]
    pub name: Option<String>,
    /// Network usage inbound + outbound
    #[serde(rename = "NetIO")]
    pub net_io: Option<String>,
    /// Container PID
    #[serde(rename = "PIDS")]
    pub pids: Option<String>,
    /// Pod ID
    #[serde(rename = "Pod")]
    pub pod: Option<String>,
}

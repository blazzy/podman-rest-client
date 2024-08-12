use serde::{Deserialize, Serialize};
#[derive(Default, Debug, Serialize, Deserialize)]
/// Statistics for an individual container network interface
pub struct ContainerNetworkStats {
    #[serde(rename = "RxBytes")]
    pub rx_bytes: Option<u64>,
    #[serde(rename = "RxDropped")]
    pub rx_dropped: Option<u64>,
    #[serde(rename = "RxErrors")]
    pub rx_errors: Option<u64>,
    #[serde(rename = "RxPackets")]
    pub rx_packets: Option<u64>,
    #[serde(rename = "TxBytes")]
    pub tx_bytes: Option<u64>,
    #[serde(rename = "TxDropped")]
    pub tx_dropped: Option<u64>,
    #[serde(rename = "TxErrors")]
    pub tx_errors: Option<u64>,
    #[serde(rename = "TxPackets")]
    pub tx_packets: Option<u64>,
}

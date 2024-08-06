use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Serialize, Deserialize)]
/// InspectBlkioThrottleDevice holds information about a speed cap for a device
/// node. This cap applies to a specific operation (read, write, etc) on the given
/// node.
pub struct InspectBlkioThrottleDevice {
    /// Path is the path to the device this applies to.
    #[serde(rename = "Path")]
    pub path: Option<String>,

    /// Rate is the maximum rate. It is in either bytes per second or iops
    /// per second, determined by where it is used - documentation will
    /// indicate which is appropriate.
    #[serde(rename = "Rate")]
    pub rate: Option<u64>,
}

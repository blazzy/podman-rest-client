use serde::{Deserialize, Serialize};
#[derive(Default, Debug, Serialize, Deserialize)]
/// LinuxDevice represents the mknod information for a Linux special device file
pub struct LinuxDevice {
    #[serde(rename = "fileMode")]
    pub file_mode: Option<u32>,
    /// Gid of the device.
    pub gid: Option<u32>,
    /// Major is the device's major number.
    pub major: Option<i64>,
    /// Minor is the device's minor number.
    pub minor: Option<i64>,
    /// Path to the device.
    pub path: Option<String>,
    /// Device type, block, char, etc.
    #[serde(rename = "type")]
    pub r#type: Option<String>,
    /// UID of the device.
    pub uid: Option<u32>,
}

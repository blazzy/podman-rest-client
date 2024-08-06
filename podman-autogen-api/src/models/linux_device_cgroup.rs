use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Serialize, Deserialize)]
/// LinuxDeviceCgroup represents a device rule for the devices specified to
/// the device controller
pub struct LinuxDeviceCgroup {
    /// Cgroup access permissions format, rwm.
    pub access: Option<String>,

    /// Allow or deny
    pub allow: Option<bool>,

    /// Major is the device's major number.
    pub major: Option<i64>,

    /// Minor is the device's minor number.
    pub minor: Option<i64>,

    /// Device type, block, char, etc.
    #[serde(rename = "type")]
    pub r#type: Option<String>,
}

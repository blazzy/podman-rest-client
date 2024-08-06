use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Serialize, Deserialize)]
/// LinuxBlockIODevice holds major:minor format supported in blkio cgroup
pub struct LinuxBlockIoDevice {
    /// Major is the device's major number.
    pub major: Option<i64>,

    /// Minor is the device's minor number.
    pub minor: Option<i64>,
}

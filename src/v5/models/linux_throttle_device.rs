use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Serialize, Deserialize)]
/// LinuxThrottleDevice struct holds a `major:minor rate_per_second` pair
pub struct LinuxThrottleDevice {
    /// Major is the device's major number.
    pub major: Option<i64>,

    /// Minor is the device's minor number.
    pub minor: Option<i64>,

    /// Rate is the IO rate limit per cgroup per device
    pub rate: Option<u64>,
}

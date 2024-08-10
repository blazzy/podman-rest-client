use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Serialize, Deserialize)]
/// LinuxWeightDevice struct holds a `major:minor weight` pair for weightDevice
pub struct LinuxWeightDevice {
    /// LeafWeight is the bandwidth rate for the device while competing with the cgroup's child cgroups, CFQ scheduler only
    #[serde(rename = "leafWeight")]
    pub leaf_weight: Option<u16>,

    /// Major is the device's major number.
    pub major: Option<i64>,

    /// Minor is the device's minor number.
    pub minor: Option<i64>,

    /// Weight is the bandwidth rate for the device.
    pub weight: Option<u16>,
}

use serde::{Deserialize, Serialize};
#[derive(Default, Debug, Serialize, Deserialize)]
/// InspectBlkioWeightDevice holds information about the relative weight
/// of an individual device node. Weights are used in the I/O scheduler to give
/// relative priority to some accesses.
pub struct InspectBlkioWeightDevice {
    /// Path is the path to the device this applies to.
    #[serde(rename = "Path")]
    pub path: Option<String>,
    /// Weight is the relative weight the scheduler will use when scheduling
    /// I/O.
    #[serde(rename = "Weight")]
    pub weight: Option<u16>,
}

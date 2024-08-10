use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Serialize, Deserialize)]
/// WeightDevice is a structure that holds device:weight pair
pub struct WeightDevice {
    #[serde(rename = "Path")]
    pub path: Option<String>,

    #[serde(rename = "Weight")]
    pub weight: Option<u16>,
}

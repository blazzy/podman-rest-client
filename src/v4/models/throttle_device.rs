use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Serialize, Deserialize)]
/// ThrottleDevice is a structure that holds device:rate_per_second pair
pub struct ThrottleDevice {
    #[serde(rename = "Path")]
    pub path: Option<String>,

    #[serde(rename = "Rate")]
    pub rate: Option<u64>,
}

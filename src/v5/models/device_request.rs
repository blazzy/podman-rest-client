use serde::{Deserialize, Serialize};
#[derive(Default, Debug, Serialize, Deserialize)]
/// DeviceRequest represents a request for devices from a device driver.
/// Used by GPU device drivers.
pub struct DeviceRequest {
    #[serde(rename = "Capabilities")]
    pub capabilities: Option<Vec<Vec<String>>>,
    #[serde(rename = "Count")]
    pub count: Option<i64>,
    #[serde(rename = "DeviceIDs")]
    pub device_i_ds: Option<Vec<String>>,
    #[serde(rename = "Driver")]
    pub driver: Option<String>,
    #[serde(rename = "Options")]
    pub options: Option<std::collections::HashMap<String, String>>,
}

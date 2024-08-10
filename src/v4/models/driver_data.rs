use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Serialize, Deserialize)]
/// DriverData handles the data for a storage driver
pub struct DriverData {
    #[serde(rename = "Data")]
    pub data: Option<std::collections::HashMap<String, String>>,

    #[serde(rename = "Name")]
    pub name: Option<String>,
}

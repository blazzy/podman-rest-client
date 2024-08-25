use serde::{Deserialize, Serialize};
#[derive(Default, Debug, Serialize, Deserialize)]
/// VolumeOptions represents the options for a mount of type volume.
pub struct VolumeOptions {
    #[serde(rename = "DriverConfig")]
    pub driver_config: Option<crate::v5::models::Driver>,
    #[serde(rename = "Labels")]
    pub labels: Option<std::collections::HashMap<String, Option<String>>>,
    #[serde(rename = "NoCopy")]
    pub no_copy: Option<bool>,
    #[serde(rename = "Subpath")]
    pub subpath: Option<String>,
}

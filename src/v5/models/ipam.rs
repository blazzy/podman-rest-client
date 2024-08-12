use serde::{Deserialize, Serialize};
#[derive(Default, Debug, Serialize, Deserialize)]
/// IPAM represents IP Address Management
pub struct Ipam {
    #[serde(rename = "Config")]
    pub config: Option<Vec<super::super::models::IpamConfig>>,
    #[serde(rename = "Driver")]
    pub driver: Option<String>,
    #[serde(rename = "Options")]
    pub options: Option<std::collections::HashMap<String, String>>,
}

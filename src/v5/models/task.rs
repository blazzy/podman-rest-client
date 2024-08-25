use serde::{Deserialize, Serialize};
#[derive(Default, Debug, Serialize, Deserialize)]
/// Task carries the information about one backend task
pub struct Task {
    #[serde(rename = "EndpointID")]
    pub endpoint_id: Option<String>,
    #[serde(rename = "EndpointIP")]
    pub endpoint_ip: Option<String>,
    #[serde(rename = "Info")]
    pub info: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "Name")]
    pub name: Option<String>,
}

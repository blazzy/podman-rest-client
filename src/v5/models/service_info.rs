use serde::{Deserialize, Serialize};
#[derive(Default, Debug, Serialize, Deserialize)]
/// ServiceInfo represents service parameters with the list of service's tasks
pub struct ServiceInfo {
    #[serde(rename = "LocalLBIndex")]
    pub local_lb_index: Option<i64>,
    #[serde(rename = "Ports")]
    pub ports: Option<Vec<String>>,
    #[serde(rename = "Tasks")]
    pub tasks: Option<Vec<crate::v5::models::Task>>,
    #[serde(rename = "VIP")]
    pub vip: Option<String>,
}

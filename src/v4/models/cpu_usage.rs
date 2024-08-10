use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Serialize, Deserialize)]
pub struct CpuUsage {
    #[serde(rename = "idlePercent")]
    pub idle_percent: Option<f64>,

    #[serde(rename = "systemPercent")]
    pub system_percent: Option<f64>,

    #[serde(rename = "userPercent")]
    pub user_percent: Option<f64>,
}

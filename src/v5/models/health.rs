use serde::{Deserialize, Serialize};
#[derive(Default, Debug, Serialize, Deserialize)]
/// Health stores information about the container's healthcheck results
pub struct Health {
    #[serde(rename = "FailingStreak")]
    pub failing_streak: Option<i64>,
    #[serde(rename = "Log")]
    pub log: Option<Vec<super::super::models::HealthcheckResult>>,
    #[serde(rename = "Status")]
    pub status: Option<String>,
}

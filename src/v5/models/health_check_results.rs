use serde::{Deserialize, Serialize};
#[derive(Default, Debug, Serialize, Deserialize)]
/// HealthCheckResults describes the results/logs from a healthcheck
pub struct HealthCheckResults {
    /// FailingStreak is the number of consecutive failed healthchecks
    #[serde(rename = "FailingStreak")]
    pub failing_streak: Option<i64>,
    /// Log describes healthcheck attempts and results
    #[serde(rename = "Log")]
    pub log: Option<Vec<crate::v5::models::HealthCheckLog>>,
    /// Status starting, healthy or unhealthy
    #[serde(rename = "Status")]
    pub status: Option<String>,
}

use serde::{Deserialize, Serialize};
#[derive(Default, Debug, Serialize, Deserialize)]
/// HealthcheckResult stores information about a single run of a healthcheck probe
pub struct HealthcheckResult {
    #[serde(rename = "End")]
    pub end: Option<String>,
    #[serde(rename = "ExitCode")]
    pub exit_code: Option<i64>,
    #[serde(rename = "Output")]
    pub output: Option<String>,
    #[serde(rename = "Start")]
    pub start: Option<String>,
}

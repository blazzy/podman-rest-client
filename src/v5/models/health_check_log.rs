use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Serialize, Deserialize)]
/// HealthCheckLog describes the results of a single healthcheck
pub struct HealthCheckLog {
    /// End time as a string
    #[serde(rename = "End")]
    pub end: Option<String>,

    /// Exitcode is 0 or 1
    #[serde(rename = "ExitCode")]
    pub exit_code: Option<i64>,

    /// Output is the stdout/stderr from the healthcheck command
    #[serde(rename = "Output")]
    pub output: Option<String>,

    /// Start time as string
    #[serde(rename = "Start")]
    pub start: Option<String>,
}

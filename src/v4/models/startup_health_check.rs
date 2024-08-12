use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Serialize, Deserialize)]
/// StartupHealthCheck is the configuration of a startup healthcheck.
pub struct StartupHealthCheck {
    #[serde(rename = "Interval")]
    pub interval: Option<i64>,

    /// Retries is the number of consecutive failures needed to consider a container as unhealthy.
    /// Zero means inherit.
    #[serde(rename = "Retries")]
    pub retries: Option<i64>,

    #[serde(rename = "StartInterval")]
    pub start_interval: Option<i64>,

    #[serde(rename = "StartPeriod")]
    pub start_period: Option<i64>,

    /// Successes are the number of successes required to mark the startup HC
    /// as passed.
    /// If set to 0, a single success will mark the HC as passed.
    #[serde(rename = "Successes")]
    pub successes: Option<i64>,

    /// Test is the test to perform to check that the container is healthy.
    /// An empty slice means to inherit the default.
    /// The options are:
    /// {} : inherit healthcheck
    /// {"NONE"} : disable healthcheck
    /// {"CMD", args...} : exec arguments directly
    /// {"CMD-SHELL", command} : run command with system's default shell
    #[serde(rename = "Test")]
    pub test: Option<Vec<String>>,

    #[serde(rename = "Timeout")]
    pub timeout: Option<i64>,
}
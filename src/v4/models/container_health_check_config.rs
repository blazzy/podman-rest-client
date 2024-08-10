use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Serialize, Deserialize)]
/// ContainerHealthCheckConfig describes a container healthcheck with attributes
/// like command, retries, interval, start period, and timeout.
pub struct ContainerHealthCheckConfig {
    pub health_check_on_failure_action: Option<i64>,

    pub healthconfig: Option<super::super::models::Schema2HealthConfig>,

    #[serde(rename = "startupHealthConfig")]
    pub startup_health_config: Option<super::super::models::StartupHealthCheck>,
}

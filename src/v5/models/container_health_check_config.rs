use serde::{Deserialize, Serialize};
#[derive(Default, Debug, Serialize, Deserialize)]
/// ContainerHealthCheckConfig describes a container healthcheck with attributes
/// like command, retries, interval, start period, and timeout.
pub struct ContainerHealthCheckConfig {
    pub health_check_on_failure_action: Option<i64>,
    pub healthconfig: Option<crate::v5::models::Schema2HealthConfig>,
    #[serde(rename = "startupHealthConfig")]
    pub startup_health_config: Option<crate::v5::models::StartupHealthCheck>,
}

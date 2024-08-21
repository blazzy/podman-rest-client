use serde::{Deserialize, Serialize};
#[derive(Default, Debug, Serialize, Deserialize)]
/// ContainerState stores container's running state
/// it's part of ContainerJSONBase and will return by "inspect" command
pub struct ContainerState {
    #[serde(rename = "Dead")]
    pub dead: Option<bool>,
    #[serde(rename = "Error")]
    pub error: Option<String>,
    #[serde(rename = "ExitCode")]
    pub exit_code: Option<i64>,
    #[serde(rename = "FinishedAt")]
    pub finished_at: Option<String>,
    #[serde(rename = "Health")]
    pub health: Option<crate::v5::models::Health>,
    #[serde(rename = "OOMKilled")]
    pub oom_killed: Option<bool>,
    #[serde(rename = "Paused")]
    pub paused: Option<bool>,
    #[serde(rename = "Pid")]
    pub pid: Option<i64>,
    #[serde(rename = "Restarting")]
    pub restarting: Option<bool>,
    #[serde(rename = "Running")]
    pub running: Option<bool>,
    #[serde(rename = "StartedAt")]
    pub started_at: Option<String>,
    #[serde(rename = "Status")]
    pub status: Option<String>,
}

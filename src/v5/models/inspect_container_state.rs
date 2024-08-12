use serde::{Deserialize, Serialize};
#[derive(Default, Debug, Serialize, Deserialize)]
/// InspectContainerState provides a detailed record of a container's current
/// state. It is returned as part of InspectContainerData.
/// As with InspectContainerData, many portions of this struct are matched to
/// Docker, but here we see more fields that are unused (nonsensical in the
/// context of Libpod).
pub struct InspectContainerState {
    #[serde(rename = "CgroupPath")]
    pub cgroup_path: Option<String>,
    #[serde(rename = "CheckpointLog")]
    pub checkpoint_log: Option<String>,
    #[serde(rename = "CheckpointPath")]
    pub checkpoint_path: Option<String>,
    #[serde(rename = "Checkpointed")]
    pub checkpointed: Option<bool>,
    #[serde(rename = "CheckpointedAt")]
    pub checkpointed_at: Option<String>,
    #[serde(rename = "ConmonPid")]
    pub conmon_pid: Option<i64>,
    #[serde(rename = "Dead")]
    pub dead: Option<bool>,
    #[serde(rename = "Error")]
    pub error: Option<String>,
    #[serde(rename = "ExitCode")]
    pub exit_code: Option<i32>,
    #[serde(rename = "FinishedAt")]
    pub finished_at: Option<String>,
    #[serde(rename = "Health")]
    pub health: Option<super::super::models::HealthCheckResults>,
    #[serde(rename = "OOMKilled")]
    pub oom_killed: Option<bool>,
    #[serde(rename = "OciVersion")]
    pub oci_version: Option<String>,
    #[serde(rename = "Paused")]
    pub paused: Option<bool>,
    #[serde(rename = "Pid")]
    pub pid: Option<i64>,
    #[serde(rename = "Restarting")]
    pub restarting: Option<bool>,
    #[serde(rename = "RestoreLog")]
    pub restore_log: Option<String>,
    #[serde(rename = "Restored")]
    pub restored: Option<bool>,
    #[serde(rename = "RestoredAt")]
    pub restored_at: Option<String>,
    #[serde(rename = "Running")]
    pub running: Option<bool>,
    #[serde(rename = "StartedAt")]
    pub started_at: Option<String>,
    #[serde(rename = "Status")]
    pub status: Option<String>,
    #[serde(rename = "StoppedByUser")]
    pub stopped_by_user: Option<bool>,
}

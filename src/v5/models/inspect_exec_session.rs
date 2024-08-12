use serde::{Deserialize, Serialize};
#[derive(Default, Debug, Serialize, Deserialize)]
/// InspectExecSession contains information about a given exec session.
pub struct InspectExecSession {
    /// CanRemove is legacy and used purely for compatibility reasons.
    /// Will always be set to true, unless the exec session is running.
    #[serde(rename = "CanRemove")]
    pub can_remove: Option<bool>,
    /// ContainerID is the ID of the container this exec session is attached
    /// to.
    #[serde(rename = "ContainerID")]
    pub container_id: Option<String>,
    /// DetachKeys are the detach keys used by the exec session.
    /// If set to "" the default keys are being used.
    /// Will show "<none>" if no detach keys are set.
    #[serde(rename = "DetachKeys")]
    pub detach_keys: Option<String>,
    /// ExitCode is the exit code of the exec session. Will be set to 0 if
    /// the exec session has not yet exited.
    #[serde(rename = "ExitCode")]
    pub exit_code: Option<i64>,
    /// ID is the ID of the exec session.
    #[serde(rename = "ID")]
    pub id: Option<String>,
    /// OpenStderr is whether the container's STDERR stream will be attached.
    /// Always set to true if the exec session created a TTY.
    #[serde(rename = "OpenStderr")]
    pub open_stderr: Option<bool>,
    /// OpenStdin is whether the container's STDIN stream will be attached
    /// to.
    #[serde(rename = "OpenStdin")]
    pub open_stdin: Option<bool>,
    /// OpenStdout is whether the container's STDOUT stream will be attached.
    /// Always set to true if the exec session created a TTY.
    #[serde(rename = "OpenStdout")]
    pub open_stdout: Option<bool>,
    /// Pid is the PID of the exec session's process.
    /// Will be set to 0 if the exec session is not running.
    #[serde(rename = "Pid")]
    pub pid: Option<i64>,
    #[serde(rename = "ProcessConfig")]
    pub process_config: Option<super::super::models::InspectExecProcess>,
    /// Running is whether the exec session is running.
    #[serde(rename = "Running")]
    pub running: Option<bool>,
}

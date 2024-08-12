use serde::{Deserialize, Serialize};
#[derive(Default, Debug, Serialize, Deserialize)]
/// InspectExecProcess contains information about the process in a given exec
/// session.
pub struct InspectExecProcess {
    /// Arguments are the arguments to the entrypoint command of the exec
    /// session.
    pub arguments: Option<Vec<String>>,
    /// Entrypoint is the entrypoint for the exec session (the command that
    /// will be executed in the container).
    pub entrypoint: Option<String>,
    /// Privileged is whether the exec session will be started with elevated
    /// privileges.
    pub privileged: Option<bool>,
    /// Tty is whether the exec session created a terminal.
    pub tty: Option<bool>,
    /// User is the user the exec session was started as.
    pub user: Option<String>,
}

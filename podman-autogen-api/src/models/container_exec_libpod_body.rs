use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Serialize, Deserialize)]
pub struct ContainerExecLibpodBody {
    /// Attach to stderr of the exec command
    #[serde(rename = "AttachStderr")]
    pub attach_stderr: Option<bool>,

    /// Attach to stdin of the exec command
    #[serde(rename = "AttachStdin")]
    pub attach_stdin: Option<bool>,

    /// Attach to stdout of the exec command
    #[serde(rename = "AttachStdout")]
    pub attach_stdout: Option<bool>,

    /// Command to run, as a string or array of strings.
    #[serde(rename = "Cmd")]
    pub cmd: Option<Vec<String>>,

    /// "Override the key sequence for detaching a container. Format is a single character [a-Z] or ctrl-<value> where <value> is one of: a-z, @, ^, [, , or _."
    #[serde(rename = "DetachKeys")]
    pub detach_keys: Option<String>,

    /// A list of environment variables in the form ["VAR=value", ...]
    #[serde(rename = "Env")]
    pub env: Option<Vec<String>>,

    /// Runs the exec process with extended privileges
    #[serde(rename = "Privileged")]
    pub privileged: Option<bool>,

    /// Allocate a pseudo-TTY
    #[serde(rename = "Tty")]
    pub tty: Option<bool>,

    /// "The user, and optionally, group to run the exec process inside the container. Format is one of: user, user:group, uid, or uid:gid."
    #[serde(rename = "User")]
    pub user: Option<String>,

    /// The working directory for the exec process inside the container.
    #[serde(rename = "WorkingDir")]
    pub working_dir: Option<String>,
}

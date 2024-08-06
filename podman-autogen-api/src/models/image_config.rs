use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Serialize, Deserialize)]
/// ImageConfig defines the execution parameters which should be used as a base when running a container using an image.
pub struct ImageConfig {
    /// ArgsEscaped
    ///
    /// Deprecated: This field is present only for legacy compatibility with
    /// Docker and should not be used by new image builders.  It is used by Docker
    /// for Windows images to indicate that the `Entrypoint` or `Cmd` or both,
    /// contains only a single element array, that is a pre-escaped, and combined
    /// into a single string `CommandLine`. If `true` the value in `Entrypoint` or
    /// `Cmd` should be used as-is to avoid double escaping.
    /// https://github.com/opencontainers/image-spec/pull/892
    #[serde(rename = "ArgsEscaped")]
    pub args_escaped: Option<bool>,

    /// Cmd defines the default arguments to the entrypoint of the container.
    #[serde(rename = "Cmd")]
    pub cmd: Option<Vec<String>>,

    /// Entrypoint defines a list of arguments to use as the command to execute when the container starts.
    #[serde(rename = "Entrypoint")]
    pub entrypoint: Option<Vec<String>>,

    /// Env is a list of environment variables to be used in a container.
    #[serde(rename = "Env")]
    pub env: Option<Vec<String>>,

    /// ExposedPorts a set of ports to expose from a container running this image.
    #[serde(rename = "ExposedPorts")]
    pub exposed_ports: Option<std::collections::HashMap<String, serde_json::Value>>,

    /// Labels contains arbitrary metadata for the container.
    #[serde(rename = "Labels")]
    pub labels: Option<std::collections::HashMap<String, String>>,

    /// StopSignal contains the system call signal that will be sent to the container to exit.
    #[serde(rename = "StopSignal")]
    pub stop_signal: Option<String>,

    /// User defines the username or UID which the process in the container should run as.
    #[serde(rename = "User")]
    pub user: Option<String>,

    /// Volumes is a set of directories describing where the process is likely write data specific to a container instance.
    #[serde(rename = "Volumes")]
    pub volumes: Option<std::collections::HashMap<String, serde_json::Value>>,

    /// WorkingDir sets the current working directory of the entrypoint process in the container.
    #[serde(rename = "WorkingDir")]
    pub working_dir: Option<String>,
}

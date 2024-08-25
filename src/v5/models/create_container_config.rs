use serde::{Deserialize, Serialize};
#[derive(Default, Debug, Serialize, Deserialize)]
/// CreateContainerConfig used when compatible endpoint creates a container
pub struct CreateContainerConfig {
    #[serde(rename = "ArgsEscaped")]
    pub args_escaped: Option<bool>,
    #[serde(rename = "AttachStderr")]
    pub attach_stderr: Option<bool>,
    #[serde(rename = "AttachStdin")]
    pub attach_stdin: Option<bool>,
    #[serde(rename = "AttachStdout")]
    pub attach_stdout: Option<bool>,
    #[serde(rename = "Cmd")]
    pub cmd: Option<Vec<String>>,
    #[serde(rename = "Domainname")]
    pub domainname: Option<String>,
    #[serde(rename = "Entrypoint")]
    pub entrypoint: Option<Vec<String>>,
    #[serde(rename = "Env")]
    pub env: Option<Vec<String>>,
    #[serde(rename = "EnvMerge")]
    pub env_merge: Option<Vec<String>>,
    #[serde(rename = "ExposedPorts")]
    pub exposed_ports: Option<std::collections::HashMap<String, Option<serde_json::Value>>>,
    #[serde(rename = "Healthcheck")]
    pub healthcheck: Option<crate::v5::models::HealthcheckConfig>,
    #[serde(rename = "HostConfig")]
    pub host_config: Option<crate::v5::models::HostConfig>,
    #[serde(rename = "Hostname")]
    pub hostname: Option<String>,
    #[serde(rename = "Image")]
    pub image: Option<String>,
    #[serde(rename = "Labels")]
    pub labels: Option<std::collections::HashMap<String, Option<String>>>,
    /// Mac Address of the container.
    ///
    /// Deprecated: this field is deprecated since API v1.44. Use EndpointSettings.MacAddress instead.
    #[serde(rename = "MacAddress")]
    pub mac_address: Option<String>,
    #[serde(rename = "Name")]
    pub name: Option<String>,
    #[serde(rename = "NetworkDisabled")]
    pub network_disabled: Option<bool>,
    #[serde(rename = "NetworkingConfig")]
    pub networking_config: Option<crate::v5::models::NetworkingConfig>,
    #[serde(rename = "OnBuild")]
    pub on_build: Option<Vec<String>>,
    #[serde(rename = "OpenStdin")]
    pub open_stdin: Option<bool>,
    #[serde(rename = "Shell")]
    pub shell: Option<Vec<String>>,
    #[serde(rename = "StdinOnce")]
    pub stdin_once: Option<bool>,
    #[serde(rename = "StopSignal")]
    pub stop_signal: Option<String>,
    #[serde(rename = "StopTimeout")]
    pub stop_timeout: Option<i64>,
    #[serde(rename = "Tty")]
    pub tty: Option<bool>,
    #[serde(rename = "UnsetEnv")]
    pub unset_env: Option<Vec<String>>,
    #[serde(rename = "UnsetEnvAll")]
    pub unset_env_all: Option<bool>,
    #[serde(rename = "User")]
    pub user: Option<String>,
    #[serde(rename = "Volumes")]
    pub volumes: Option<std::collections::HashMap<String, Option<serde_json::Value>>>,
    #[serde(rename = "WorkingDir")]
    pub working_dir: Option<String>,
}

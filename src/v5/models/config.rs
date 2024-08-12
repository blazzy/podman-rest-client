use serde::{Deserialize, Serialize};
#[derive(Default, Debug, Serialize, Deserialize)]
/// Config contains the configuration data about a container.
/// It should hold only portable information about the container.
/// Here, "portable" means "independent from the host we are running on".
/// Non-portable information *should* appear in HostConfig.
/// All fields added to this struct must be marked `omitempty` to keep getting
/// predictable hashes from the old `v1Compatibility` configuration.
pub struct Config {
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
    #[serde(rename = "ExposedPorts")]
    pub exposed_ports: Option<std::collections::HashMap<String, serde_json::Value>>,
    #[serde(rename = "Healthcheck")]
    pub healthcheck: Option<super::super::models::HealthcheckConfig>,
    #[serde(rename = "Hostname")]
    pub hostname: Option<String>,
    #[serde(rename = "Image")]
    pub image: Option<String>,
    #[serde(rename = "Labels")]
    pub labels: Option<std::collections::HashMap<String, String>>,
    /// Mac Address of the container.
    ///
    /// Deprecated: this field is deprecated since API v1.44. Use EndpointSettings.MacAddress instead.
    #[serde(rename = "MacAddress")]
    pub mac_address: Option<String>,
    #[serde(rename = "NetworkDisabled")]
    pub network_disabled: Option<bool>,
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
    #[serde(rename = "User")]
    pub user: Option<String>,
    #[serde(rename = "Volumes")]
    pub volumes: Option<std::collections::HashMap<String, serde_json::Value>>,
    #[serde(rename = "WorkingDir")]
    pub working_dir: Option<String>,
}

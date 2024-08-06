use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Serialize, Deserialize)]
/// PluginConfig The config of a plugin.
pub struct PluginConfig {
    #[serde(rename = "Args")]
    pub args: super::super::models::PluginConfigArgs,

    /// description
    #[serde(rename = "Description")]
    pub description: String,

    /// Docker Version used to create the plugin
    #[serde(rename = "DockerVersion")]
    pub docker_version: Option<String>,

    /// documentation
    #[serde(rename = "Documentation")]
    pub documentation: String,

    /// entrypoint
    #[serde(rename = "Entrypoint")]
    pub entrypoint: Vec<String>,

    /// env
    #[serde(rename = "Env")]
    pub env: Vec<super::super::models::PluginEnv>,

    #[serde(rename = "Interface")]
    pub interface: super::super::models::PluginConfigInterface,

    /// ipc host
    #[serde(rename = "IpcHost")]
    pub ipc_host: bool,

    #[serde(rename = "Linux")]
    pub linux: super::super::models::PluginConfigLinux,

    /// mounts
    #[serde(rename = "Mounts")]
    pub mounts: Vec<super::super::models::PluginMount>,

    #[serde(rename = "Network")]
    pub network: super::super::models::PluginConfigNetwork,

    /// pid host
    #[serde(rename = "PidHost")]
    pub pid_host: bool,

    /// propagated mount
    #[serde(rename = "PropagatedMount")]
    pub propagated_mount: String,

    #[serde(rename = "User")]
    pub user: Option<super::super::models::PluginConfigUser>,

    /// work dir
    #[serde(rename = "WorkDir")]
    pub work_dir: String,

    pub rootfs: Option<super::super::models::PluginConfigRootfs>,
}

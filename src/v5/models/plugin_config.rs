use serde::{Deserialize, Serialize};
#[derive(Default, Debug, Serialize, Deserialize)]
/// PluginConfig The config of a plugin.
pub struct PluginConfig {
    #[serde(rename = "Args")]
    pub args: crate::v5::models::PluginConfigArgs,
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
    pub env: Vec<crate::v5::models::PluginEnv>,
    #[serde(rename = "Interface")]
    pub interface: crate::v5::models::PluginConfigInterface,
    /// ipc host
    #[serde(rename = "IpcHost")]
    pub ipc_host: bool,
    #[serde(rename = "Linux")]
    pub linux: crate::v5::models::PluginConfigLinux,
    /// mounts
    #[serde(rename = "Mounts")]
    pub mounts: Vec<crate::v5::models::PluginMount>,
    #[serde(rename = "Network")]
    pub network: crate::v5::models::PluginConfigNetwork,
    /// pid host
    #[serde(rename = "PidHost")]
    pub pid_host: bool,
    /// propagated mount
    #[serde(rename = "PropagatedMount")]
    pub propagated_mount: String,
    #[serde(rename = "User")]
    pub user: Option<crate::v5::models::PluginConfigUser>,
    /// work dir
    #[serde(rename = "WorkDir")]
    pub work_dir: String,
    pub rootfs: Option<crate::v5::models::PluginConfigRootfs>,
}

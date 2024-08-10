use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Serialize, Deserialize)]
/// ContainerJSON is newly used struct along with MountPoint
pub struct ContainerJson {
    #[serde(rename = "AppArmorProfile")]
    pub app_armor_profile: Option<String>,

    #[serde(rename = "Args")]
    pub args: Option<Vec<String>>,

    #[serde(rename = "Config")]
    pub config: Option<super::super::models::Config>,

    #[serde(rename = "Created")]
    pub created: Option<String>,

    #[serde(rename = "Driver")]
    pub driver: Option<String>,

    #[serde(rename = "ExecIDs")]
    pub exec_i_ds: Option<Vec<String>>,

    #[serde(rename = "GraphDriver")]
    pub graph_driver: Option<super::super::models::GraphDriverData>,

    #[serde(rename = "HostConfig")]
    pub host_config: Option<super::super::models::HostConfig>,

    #[serde(rename = "HostnamePath")]
    pub hostname_path: Option<String>,

    #[serde(rename = "HostsPath")]
    pub hosts_path: Option<String>,

    #[serde(rename = "Id")]
    pub id: Option<String>,

    #[serde(rename = "Image")]
    pub image: Option<String>,

    #[serde(rename = "LogPath")]
    pub log_path: Option<String>,

    #[serde(rename = "MountLabel")]
    pub mount_label: Option<String>,

    #[serde(rename = "Mounts")]
    pub mounts: Option<Vec<super::super::models::MountPoint>>,

    #[serde(rename = "Name")]
    pub name: Option<String>,

    #[serde(rename = "NetworkSettings")]
    pub network_settings: Option<super::super::models::NetworkSettings>,

    #[serde(rename = "Node")]
    pub node: Option<super::super::models::ContainerNode>,

    #[serde(rename = "Path")]
    pub path: Option<String>,

    #[serde(rename = "Platform")]
    pub platform: Option<String>,

    #[serde(rename = "ProcessLabel")]
    pub process_label: Option<String>,

    #[serde(rename = "ResolvConfPath")]
    pub resolv_conf_path: Option<String>,

    #[serde(rename = "RestartCount")]
    pub restart_count: Option<i64>,

    #[serde(rename = "SizeRootFs")]
    pub size_root_fs: Option<i64>,

    #[serde(rename = "SizeRw")]
    pub size_rw: Option<i64>,

    #[serde(rename = "State")]
    pub state: Option<super::super::models::ContainerState>,
}

use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Serialize, Deserialize)]
/// InspectContainerData provides a detailed record of a container's configuration
/// and state as viewed by Libpod.
/// Large portions of this structure are defined such that the output is
/// compatible with `docker inspect` JSON, but additional fields have been added
/// as required to share information not in the original output.
pub struct InspectContainerData {
    #[serde(rename = "AppArmorProfile")]
    pub app_armor_profile: Option<String>,

    #[serde(rename = "Args")]
    pub args: Option<Vec<String>>,

    #[serde(rename = "BoundingCaps")]
    pub bounding_caps: Option<Vec<String>>,

    #[serde(rename = "Config")]
    pub config: Option<super::super::models::InspectContainerConfig>,

    #[serde(rename = "ConmonPidFile")]
    pub conmon_pid_file: Option<String>,

    #[serde(rename = "Created")]
    pub created: Option<String>,

    #[serde(rename = "Dependencies")]
    pub dependencies: Option<Vec<String>>,

    #[serde(rename = "Driver")]
    pub driver: Option<String>,

    #[serde(rename = "EffectiveCaps")]
    pub effective_caps: Option<Vec<String>>,

    #[serde(rename = "ExecIDs")]
    pub exec_i_ds: Option<Vec<String>>,

    #[serde(rename = "GraphDriver")]
    pub graph_driver: Option<super::super::models::DriverData>,

    #[serde(rename = "HostConfig")]
    pub host_config: Option<super::super::models::InspectContainerHostConfig>,

    #[serde(rename = "HostnamePath")]
    pub hostname_path: Option<String>,

    #[serde(rename = "HostsPath")]
    pub hosts_path: Option<String>,

    #[serde(rename = "Id")]
    pub id: Option<String>,

    #[serde(rename = "Image")]
    pub image: Option<String>,

    #[serde(rename = "ImageDigest")]
    pub image_digest: Option<String>,

    #[serde(rename = "ImageName")]
    pub image_name: Option<String>,

    #[serde(rename = "IsInfra")]
    pub is_infra: Option<bool>,

    #[serde(rename = "IsService")]
    pub is_service: Option<bool>,

    #[serde(rename = "KubeExitCodePropagation")]
    pub kube_exit_code_propagation: Option<String>,

    #[serde(rename = "MountLabel")]
    pub mount_label: Option<String>,

    #[serde(rename = "Mounts")]
    pub mounts: Option<Vec<super::super::models::InspectMount>>,

    #[serde(rename = "Name")]
    pub name: Option<String>,

    #[serde(rename = "Namespace")]
    pub namespace: Option<String>,

    #[serde(rename = "NetworkSettings")]
    pub network_settings: Option<super::super::models::InspectNetworkSettings>,

    #[serde(rename = "OCIConfigPath")]
    pub oci_config_path: Option<String>,

    #[serde(rename = "OCIRuntime")]
    pub oci_runtime: Option<String>,

    #[serde(rename = "Path")]
    pub path: Option<String>,

    #[serde(rename = "PidFile")]
    pub pid_file: Option<String>,

    #[serde(rename = "Pod")]
    pub pod: Option<String>,

    #[serde(rename = "ProcessLabel")]
    pub process_label: Option<String>,

    #[serde(rename = "ResolvConfPath")]
    pub resolv_conf_path: Option<String>,

    #[serde(rename = "RestartCount")]
    pub restart_count: Option<i32>,

    #[serde(rename = "Rootfs")]
    pub rootfs: Option<String>,

    #[serde(rename = "SizeRootFs")]
    pub size_root_fs: Option<i64>,

    #[serde(rename = "SizeRw")]
    pub size_rw: Option<i64>,

    #[serde(rename = "State")]
    pub state: Option<super::super::models::InspectContainerState>,

    #[serde(rename = "StaticDir")]
    pub static_dir: Option<String>,

    #[serde(rename = "lockNumber")]
    pub lock_number: Option<u32>,
}

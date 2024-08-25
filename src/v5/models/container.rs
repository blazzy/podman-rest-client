use serde::{Deserialize, Serialize};
#[derive(Default, Debug, Serialize, Deserialize)]
pub struct Container {
    #[serde(rename = "Command")]
    pub command: Option<String>,
    #[serde(rename = "Config")]
    pub config: Option<crate::v5::models::Config>,
    #[serde(rename = "Created")]
    pub created: Option<i64>,
    #[serde(rename = "DefaultReadOnlyNonRecursive")]
    pub default_read_only_non_recursive: Option<bool>,
    #[serde(rename = "HostConfig")]
    pub host_config: Option<crate::v5::models::HostConfig>,
    #[serde(rename = "Id")]
    pub id: Option<String>,
    #[serde(rename = "Image")]
    pub image: Option<String>,
    #[serde(rename = "ImageID")]
    pub image_id: Option<String>,
    #[serde(rename = "Labels")]
    pub labels: Option<std::collections::HashMap<String, Option<String>>>,
    #[serde(rename = "Mounts")]
    pub mounts: Option<Vec<crate::v5::models::MountPoint>>,
    #[serde(rename = "Name")]
    pub name: Option<String>,
    #[serde(rename = "Names")]
    pub names: Option<Vec<String>>,
    #[serde(rename = "NetworkSettings")]
    pub network_settings: Option<crate::v5::models::SummaryNetworkSettings>,
    #[serde(rename = "NetworkingConfig")]
    pub networking_config: Option<crate::v5::models::NetworkingConfig>,
    #[serde(rename = "Platform")]
    pub platform: Option<crate::v5::models::Platform>,
    #[serde(rename = "Ports")]
    pub ports: Option<Vec<crate::v5::models::Port>>,
    #[serde(rename = "SizeRootFs")]
    pub size_root_fs: Option<i64>,
    #[serde(rename = "SizeRw")]
    pub size_rw: Option<i64>,
    #[serde(rename = "State")]
    pub state: Option<String>,
    #[serde(rename = "Status")]
    pub status: Option<String>,
}

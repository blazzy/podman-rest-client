use serde::{Deserialize, Serialize};
#[derive(Default, Debug, Serialize, Deserialize)]
/// SystemDfContainerReport describes a container for use with df
pub struct SystemDfContainerReport {
    #[serde(rename = "Command")]
    pub command: Option<Vec<String>>,
    #[serde(rename = "ContainerID")]
    pub container_id: Option<String>,
    #[serde(rename = "Created")]
    pub created: Option<String>,
    #[serde(rename = "Image")]
    pub image: Option<String>,
    #[serde(rename = "LocalVolumes")]
    pub local_volumes: Option<i64>,
    #[serde(rename = "Names")]
    pub names: Option<String>,
    #[serde(rename = "RWSize")]
    pub rw_size: Option<i64>,
    #[serde(rename = "Size")]
    pub size: Option<i64>,
    #[serde(rename = "Status")]
    pub status: Option<String>,
}

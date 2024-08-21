use serde::{Deserialize, Serialize};
#[derive(Default, Debug, Serialize, Deserialize)]
/// StoreInfo describes the container storage and its
/// attributes
pub struct StoreInfo {
    #[serde(rename = "configFile")]
    pub config_file: Option<String>,
    #[serde(rename = "containerStore")]
    pub container_store: Option<crate::v5::models::ContainerStore>,
    #[serde(rename = "graphDriverName")]
    pub graph_driver_name: Option<String>,
    #[serde(rename = "graphOptions")]
    pub graph_options: Option<std::collections::HashMap<String, ()>>,
    #[serde(rename = "graphRoot")]
    pub graph_root: Option<String>,
    /// GraphRootAllocated is how much space the graphroot has in bytes
    #[serde(rename = "graphRootAllocated")]
    pub graph_root_allocated: Option<u64>,
    /// GraphRootUsed is how much of graphroot is used in bytes
    #[serde(rename = "graphRootUsed")]
    pub graph_root_used: Option<u64>,
    #[serde(rename = "graphStatus")]
    pub graph_status: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "imageCopyTmpDir")]
    pub image_copy_tmp_dir: Option<String>,
    #[serde(rename = "imageStore")]
    pub image_store: Option<crate::v5::models::ImageStore>,
    #[serde(rename = "runRoot")]
    pub run_root: Option<String>,
    #[serde(rename = "transientStore")]
    pub transient_store: Option<bool>,
    #[serde(rename = "volumePath")]
    pub volume_path: Option<String>,
}

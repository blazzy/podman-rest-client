use serde::{Deserialize, Serialize};
#[derive(Default, Debug, Serialize, Deserialize)]
/// ContainerSize holds the size of the container's root filesystem and top
/// read-write layer.
pub struct ContainerSize {
    #[serde(rename = "rootFsSize")]
    pub root_fs_size: Option<i64>,
    #[serde(rename = "rwSize")]
    pub rw_size: Option<i64>,
}

use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Serialize, Deserialize)]
pub struct ContainersPruneReport {
    #[serde(rename = "ContainersDeleted")]
    pub containers_deleted: Option<Vec<String>>,

    #[serde(rename = "SpaceReclaimed")]
    pub space_reclaimed: Option<u64>,
}

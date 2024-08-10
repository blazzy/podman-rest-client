use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Serialize, Deserialize)]
/// VolumesPruneReport contains the response for Engine API:
/// POST "/volumes/prune"
pub struct VolumesPruneReport {
    #[serde(rename = "SpaceReclaimed")]
    pub space_reclaimed: Option<u64>,

    #[serde(rename = "VolumesDeleted")]
    pub volumes_deleted: Option<Vec<String>>,
}

use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Serialize, Deserialize)]
/// ClusterVolume contains options and information specific to, and only present
/// on, Swarm CSI cluster volumes.
pub struct ClusterVolume {
    #[serde(rename = "CreatedAt")]
    pub created_at: Option<String>,

    /// ID is the Swarm ID of the volume. Because cluster volumes are Swarm
    /// objects, they have an ID, unlike non-cluster volumes, which only have a
    /// Name. This ID can be used to refer to the cluster volume.
    #[serde(rename = "ID")]
    pub id: Option<String>,

    #[serde(rename = "Info")]
    pub info: Option<super::super::models::Info>,

    /// PublishStatus contains the status of the volume as it pertains to its
    /// publishing on Nodes.
    #[serde(rename = "PublishStatus")]
    pub publish_status: Option<Vec<super::super::models::PublishStatus>>,

    #[serde(rename = "Spec")]
    pub spec: Option<super::super::models::ClusterVolumeSpec>,

    #[serde(rename = "UpdatedAt")]
    pub updated_at: Option<String>,

    #[serde(rename = "Version")]
    pub version: Option<super::super::models::Version>,
}

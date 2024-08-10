use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Serialize, Deserialize)]
/// ClusterVolumeSpec contains the spec used to create this volume.
pub struct ClusterVolumeSpec {
    #[serde(rename = "AccessMode")]
    pub access_mode: Option<super::super::models::AccessMode>,

    #[serde(rename = "AccessibilityRequirements")]
    pub accessibility_requirements: Option<super::super::models::TopologyRequirement>,

    #[serde(rename = "Availability")]
    pub availability: Option<String>,

    #[serde(rename = "CapacityRange")]
    pub capacity_range: Option<super::super::models::CapacityRange>,

    /// Group defines the volume group of this volume. Volumes belonging to the
    /// same group can be referred to by group name when creating Services.
    /// Referring to a volume by group instructs swarm to treat volumes in that
    /// group interchangeably for the purpose of scheduling. Volumes with an
    /// empty string for a group technically all belong to the same, emptystring
    /// group.
    #[serde(rename = "Group")]
    pub group: Option<String>,

    /// Secrets defines Swarm Secrets that are passed to the CSI storage plugin
    /// when operating on this volume.
    #[serde(rename = "Secrets")]
    pub secrets: Option<Vec<super::super::models::Secret>>,
}

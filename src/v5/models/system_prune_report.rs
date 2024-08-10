use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Serialize, Deserialize)]
/// SystemPruneReport provides report after system prune is executed.
pub struct SystemPruneReport {
    #[serde(rename = "ContainerPruneReports")]
    pub container_prune_reports: Option<Vec<super::super::models::PruneReport>>,

    #[serde(rename = "ImagePruneReports")]
    pub image_prune_reports: Option<Vec<super::super::models::PruneReport>>,

    #[serde(rename = "NetworkPruneReports")]
    pub network_prune_reports: Option<Vec<super::super::models::NetworkPruneReport>>,

    #[serde(rename = "PodPruneReport")]
    pub pod_prune_report: Option<Vec<super::super::models::PodPruneReport>>,

    #[serde(rename = "ReclaimedSpace")]
    pub reclaimed_space: Option<u64>,

    #[serde(rename = "VolumePruneReports")]
    pub volume_prune_reports: Option<Vec<super::super::models::PruneReport>>,
}

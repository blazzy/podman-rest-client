use serde::{Deserialize, Serialize};
#[derive(Default, Debug, Serialize, Deserialize)]
pub struct PlayKubeReport {
    /// If set, exit with the specified exit code.
    #[serde(rename = "ExitCode")]
    pub exit_code: Option<i32>,
    /// Pods - pods created by play kube.
    #[serde(rename = "Pods")]
    pub pods: Option<Vec<crate::v5::models::PlayKubePod>>,
    #[serde(rename = "RmReport")]
    pub rm_report: Option<Vec<crate::v5::models::PodRmReport>>,
    #[serde(rename = "SecretRmReport")]
    pub secret_rm_report: Option<Vec<crate::v5::models::SecretRmReport>>,
    /// Secrets - secrets created by play kube
    #[serde(rename = "Secrets")]
    pub secrets: Option<Vec<crate::v5::models::PlaySecret>>,
    /// ServiceContainerID - ID of the service container if one is created
    #[serde(rename = "ServiceContainerID")]
    pub service_container_id: Option<String>,
    #[serde(rename = "StopReport")]
    pub stop_report: Option<Vec<crate::v5::models::PodStopReport>>,
    #[serde(rename = "VolumeRmReport")]
    pub volume_rm_report: Option<Vec<crate::v5::models::VolumeRmReport>>,
    /// Volumes - volumes created by play kube.
    #[serde(rename = "Volumes")]
    pub volumes: Option<Vec<crate::v5::models::PlayKubeVolume>>,
}

use serde::{Deserialize, Serialize};
#[derive(Default, Debug, Serialize, Deserialize)]
pub struct PlayKubePod {
    /// ContainerErrors - any errors that occurred while starting containers
    /// in the pod.
    #[serde(rename = "ContainerErrors")]
    pub container_errors: Option<Vec<String>>,
    /// Containers - the IDs of the containers running in the created pod.
    #[serde(rename = "Containers")]
    pub containers: Option<Vec<String>>,
    /// ID - ID of the pod created as a result of play kube.
    #[serde(rename = "ID")]
    pub id: Option<String>,
    /// InitContainers - the IDs of the init containers to be run in the created pod.
    #[serde(rename = "InitContainers")]
    pub init_containers: Option<Vec<String>>,
    /// Logs - non-fatal errors and log messages while processing.
    #[serde(rename = "Logs")]
    pub logs: Option<Vec<String>>,
}

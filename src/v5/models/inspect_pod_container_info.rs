use serde::{Deserialize, Serialize};
#[derive(Default, Debug, Serialize, Deserialize)]
/// InspectPodContainerInfo contains information on a container in a pod.
pub struct InspectPodContainerInfo {
    /// ID is the ID of the container.
    #[serde(rename = "Id")]
    pub id: Option<String>,
    /// Name is the name of the container.
    #[serde(rename = "Name")]
    pub name: Option<String>,
    /// State is the current status of the container.
    #[serde(rename = "State")]
    pub state: Option<String>,
}

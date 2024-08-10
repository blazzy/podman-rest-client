use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Serialize, Deserialize)]
pub struct PlayKubeVolume {
    /// Name - Name of the volume created by play kube.
    #[serde(rename = "Name")]
    pub name: Option<String>,
}

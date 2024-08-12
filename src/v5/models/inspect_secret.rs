use serde::{Deserialize, Serialize};
#[derive(Default, Debug, Serialize, Deserialize)]
/// InspectSecret contains information on secrets mounted inside the container
pub struct InspectSecret {
    /// ID is the GID of the mounted secret file
    #[serde(rename = "GID")]
    pub gid: Option<u32>,
    /// ID is the ID of the secret
    #[serde(rename = "ID")]
    pub id: Option<String>,
    /// ID is the ID of the mode of the mounted secret file
    #[serde(rename = "Mode")]
    pub mode: Option<u32>,
    /// Name is the name of the secret
    #[serde(rename = "Name")]
    pub name: Option<String>,
    /// ID is the UID of the mounted secret file
    #[serde(rename = "UID")]
    pub uid: Option<u32>,
}

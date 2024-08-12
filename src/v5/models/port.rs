use serde::{Deserialize, Serialize};
#[derive(Default, Debug, Serialize, Deserialize)]
/// Port An open port on a container
pub struct Port {
    /// Host IP address that the container's port is mapped to
    #[serde(rename = "IP")]
    pub ip: Option<String>,
    /// Port on the container
    #[serde(rename = "PrivatePort")]
    pub private_port: u16,
    /// Port exposed on the host
    #[serde(rename = "PublicPort")]
    pub public_port: Option<u16>,
    /// type
    #[serde(rename = "Type")]
    pub r#type: String,
}

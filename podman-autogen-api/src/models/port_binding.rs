use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Serialize, Deserialize)]
/// PortBinding represents a binding between a Host IP address and a Host Port
pub struct PortBinding {
    /// HostIP is the host IP Address
    #[serde(rename = "HostIp")]
    pub host_ip: Option<String>,

    /// HostPort is the host port number
    #[serde(rename = "HostPort")]
    pub host_port: Option<String>,
}

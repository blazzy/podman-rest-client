use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Serialize, Deserialize)]
/// InspectHostPort provides information on a port on the host that a container's
/// port is bound to.
pub struct InspectHostPort {
    /// IP on the host we are bound to. "" if not specified (binding to all
    /// IPs).
    #[serde(rename = "HostIp")]
    pub host_ip: Option<String>,

    /// Port on the host we are bound to. No special formatting - just an
    /// integer stuffed into a string.
    #[serde(rename = "HostPort")]
    pub host_port: Option<String>,
}

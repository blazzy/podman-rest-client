use serde::{Deserialize, Serialize};
#[derive(Default, Debug, Serialize, Deserialize)]
/// NetworkConnectOptions describes options for connecting
/// a container to a network
pub struct NetworkConnectOptions {
    /// Aliases contains a list of names which the dns server should resolve
    /// to this container. Should only be set when DNSEnabled is true on the Network.
    /// If aliases are set but there is no dns support for this network the
    /// network interface implementation should ignore this and NOT error.
    /// Optional.
    pub aliases: Option<Vec<String>>,
    pub container: Option<String>,
    /// InterfaceName for this container. Required in the backend.
    /// Optional in the frontend. Will be filled with ethX (where X is a integer) when empty.
    pub interface_name: Option<String>,
    /// StaticIPs for this container. Optional.
    pub static_ips: Option<Vec<String>>,
    /// StaticMac for this container. Optional.
    pub static_mac: Option<String>,
}

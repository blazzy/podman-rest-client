use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Serialize, Deserialize)]
/// PortMapping is one or more ports that will be mapped into the container.
pub struct PortMapping {
    /// ContainerPort is the port number that will be exposed from the
    /// container.
    /// Mandatory.
    pub container_port: Option<u16>,

    /// HostIP is the IP that we will bind to on the host.
    /// If unset, assumed to be 0.0.0.0 (all interfaces).
    pub host_ip: Option<String>,

    /// HostPort is the port number that will be forwarded from the host into
    /// the container.
    /// If omitted, a random port on the host (guaranteed to be over 1024)
    /// will be assigned.
    pub host_port: Option<u16>,

    /// Protocol is the protocol forward.
    /// Must be either "tcp", "udp", and "sctp", or some combination of these
    /// separated by commas.
    /// If unset, assumed to be TCP.
    pub protocol: Option<String>,

    /// Range is the number of ports that will be forwarded, starting at
    /// HostPort and ContainerPort and counting up.
    /// This is 1-indexed, so 1 is assumed to be a single port (only the
    /// Hostport:Containerport mapping will be added), 2 is two ports (both
    /// Hostport:Containerport and Hostport+1:Containerport+1), etc.
    /// If unset, assumed to be 1 (a single port).
    /// Both hostport + range and containerport + range must be less than
    /// 65536.
    pub range: Option<u16>,
}

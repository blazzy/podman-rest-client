use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Serialize, Deserialize)]
/// LinuxInterfacePriority for network interfaces
pub struct LinuxInterfacePriority {
    /// Name is the name of the network interface
    pub name: Option<String>,

    /// Priority for the interface
    pub priority: Option<u32>,
}

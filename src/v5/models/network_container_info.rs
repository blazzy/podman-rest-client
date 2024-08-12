use serde::{Deserialize, Serialize};
#[derive(Default, Debug, Serialize, Deserialize)]
pub struct NetworkContainerInfo {
    /// Interfaces configured for this container with their addresses
    pub interfaces: Option<std::collections::HashMap<String, super::super::models::NetInterface>>,
    /// Name of the container
    pub name: Option<String>,
}

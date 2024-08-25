use serde::{Deserialize, Serialize};
#[derive(Default, Debug, Serialize, Deserialize)]
pub struct NetworkContainerInfo {
    /// Interfaces configured for this container with their addresses
    pub interfaces:
        Option<std::collections::HashMap<String, Option<crate::v5::models::NetInterface>>>,
    /// Name of the container
    pub name: Option<String>,
}

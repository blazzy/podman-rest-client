use serde::{Deserialize, Serialize};
#[derive(Default, Debug, Serialize, Deserialize)]
/// Info is the overall struct that describes the host system
/// running libpod/podman
pub struct LibpodInfo {
    pub host: Option<crate::v5::models::HostInfo>,
    pub plugins: Option<crate::v5::models::Plugins>,
    pub registries: Option<std::collections::HashMap<String, ()>>,
    pub store: Option<crate::v5::models::StoreInfo>,
    pub version: Option<crate::v5::models::Version>,
}

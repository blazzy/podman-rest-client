use serde::{Deserialize, Serialize};
#[derive(Default, Debug, Serialize, Deserialize)]
/// Info is the overall struct that describes the host system
/// running libpod/podman
pub struct LibpodInfo {
    pub host: Option<super::super::models::HostInfo>,
    pub plugins: Option<super::super::models::Plugins>,
    pub registries: Option<std::collections::HashMap<String, ()>>,
    pub store: Option<super::super::models::StoreInfo>,
    pub version: Option<super::super::models::Version>,
}

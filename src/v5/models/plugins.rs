use serde::{Deserialize, Serialize};
#[derive(Default, Debug, Serialize, Deserialize)]
pub struct Plugins {
    /// Authorization is provided for compatibility, will always be nil as Podman has no daemon
    pub authorization: Option<Vec<String>>,
    pub log: Option<Vec<String>>,
    pub network: Option<Vec<String>>,
    pub volume: Option<Vec<String>>,
}

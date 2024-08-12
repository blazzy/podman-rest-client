use serde::{Deserialize, Serialize};
#[derive(Default, Debug, Serialize, Deserialize)]
/// SummaryNetworkSettings provides a summary of container's networks
/// in /containers/json
pub struct SummaryNetworkSettings {
    #[serde(rename = "Networks")]
    pub networks: Option<std::collections::HashMap<String, super::super::models::EndpointSettings>>,
}

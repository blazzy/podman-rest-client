use serde::{Deserialize, Serialize};
#[derive(Default, Debug, Serialize, Deserialize)]
/// NetworkingConfig represents the container's networking configuration for each of its interfaces
/// Carries the networking configs specified in the `docker run` and `docker network connect` commands
pub struct NetworkingConfig {
    #[serde(rename = "EndpointsConfig")]
    pub endpoints_config:
        Option<std::collections::HashMap<String, super::super::models::EndpointSettings>>,
}

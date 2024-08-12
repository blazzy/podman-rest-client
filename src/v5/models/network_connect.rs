use serde::{Deserialize, Serialize};
#[derive(Default, Debug, Serialize, Deserialize)]
/// NetworkConnect represents the data to be used to connect a container to the network
pub struct NetworkConnect {
    #[serde(rename = "Container")]
    pub container: Option<String>,
    #[serde(rename = "EndpointConfig")]
    pub endpoint_config: Option<super::super::models::EndpointSettings>,
}

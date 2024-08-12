use serde::{Deserialize, Serialize};
#[derive(Default, Debug, Serialize, Deserialize)]
/// CreateResponse ContainerCreateResponse
/// OK response to ContainerCreate operation
pub struct CreateResponse {
    /// The ID of the created container
    #[serde(rename = "Id")]
    pub id: String,
    /// Warnings encountered when creating the container
    #[serde(rename = "Warnings")]
    pub warnings: Vec<String>,
}

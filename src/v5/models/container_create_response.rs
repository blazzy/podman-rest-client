use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Serialize, Deserialize)]
/// ContainerCreateResponse is the response struct for creating a container
pub struct ContainerCreateResponse {
    /// ID of the container created
    #[serde(rename = "Id")]
    pub id: String,

    /// Warnings during container creation
    #[serde(rename = "Warnings")]
    pub warnings: Vec<String>,
}

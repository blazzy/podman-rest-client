use serde::{Deserialize, Serialize};
#[derive(Default, Debug, Serialize, Deserialize)]
/// UpdateEntities used to wrap the oci resource spec in a swagger model
pub struct UpdateEntities {
    #[serde(rename = "Resources")]
    pub resources: Option<crate::v5::models::LinuxResources>,
}

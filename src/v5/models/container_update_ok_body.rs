use serde::{Deserialize, Serialize};
#[derive(Default, Debug, Serialize, Deserialize)]
/// ContainerUpdateOKBody OK response to ContainerUpdate operation
pub struct ContainerUpdateOkBody {
    /// warnings
    #[serde(rename = "Warnings")]
    pub warnings: Vec<String>,
}

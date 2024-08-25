use serde::{Deserialize, Serialize};
#[derive(Default, Debug, Serialize, Deserialize)]
pub struct ContainerExec201 {
    #[serde(rename = "Id")]
    pub id: String,
}

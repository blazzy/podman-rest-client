use serde::{Deserialize, Serialize};
#[derive(Default, Debug, Serialize, Deserialize)]
pub struct ContainerExecLibpod201 {
    #[serde(rename = "Id")]
    pub id: String,
}

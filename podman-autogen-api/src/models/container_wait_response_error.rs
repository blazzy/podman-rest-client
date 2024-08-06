use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Serialize, Deserialize)]
pub struct ContainerWaitResponseError {
    #[serde(rename = "Message")]
    pub message: Option<String>,
}

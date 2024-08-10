use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Serialize, Deserialize)]
/// RestartPolicy represents the restart policies of the container.
pub struct RestartPolicy {
    #[serde(rename = "MaximumRetryCount")]
    pub maximum_retry_count: Option<i64>,

    #[serde(rename = "Name")]
    pub name: Option<String>,
}

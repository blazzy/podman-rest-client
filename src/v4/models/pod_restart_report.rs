use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Serialize, Deserialize)]
pub struct PodRestartReport {
    #[serde(rename = "Errs")]
    pub errs: Option<Vec<String>>,

    #[serde(rename = "Id")]
    pub id: Option<String>,
}

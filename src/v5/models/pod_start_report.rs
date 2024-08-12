use serde::{Deserialize, Serialize};
#[derive(Default, Debug, Serialize, Deserialize)]
pub struct PodStartReport {
    #[serde(rename = "Errs")]
    pub errs: Option<Vec<String>>,
    #[serde(rename = "Id")]
    pub id: Option<String>,
}

use serde::{Deserialize, Serialize};
#[derive(Default, Debug, Serialize, Deserialize)]
pub struct ImageLoadReport {
    #[serde(rename = "Names")]
    pub names: Option<Vec<String>>,
}

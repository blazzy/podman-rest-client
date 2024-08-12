use serde::{Deserialize, Serialize};
#[derive(Default, Debug, Serialize, Deserialize)]
pub struct ScpReport {
    #[serde(rename = "Id")]
    pub id: Option<String>,
}

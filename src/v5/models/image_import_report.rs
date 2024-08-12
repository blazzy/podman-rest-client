use serde::{Deserialize, Serialize};
#[derive(Default, Debug, Serialize, Deserialize)]
pub struct ImageImportReport {
    #[serde(rename = "Id")]
    pub id: Option<String>,
}

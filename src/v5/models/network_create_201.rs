use serde::{Deserialize, Serialize};
#[derive(Default, Debug, Serialize, Deserialize)]
pub struct NetworkCreate201 {
    #[serde(rename = "Id")]
    pub id: Option<String>,
    #[serde(rename = "Warning")]
    pub warning: Option<String>,
}

use serde::{Deserialize, Serialize};
#[derive(Default, Debug, Serialize, Deserialize)]
/// NetworkRmReport describes the results of network removal
pub struct NetworkRmReport {
    #[serde(rename = "Err")]
    pub err: Option<String>,
    #[serde(rename = "Name")]
    pub name: Option<String>,
}

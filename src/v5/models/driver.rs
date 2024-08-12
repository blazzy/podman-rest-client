use serde::{Deserialize, Serialize};
#[derive(Default, Debug, Serialize, Deserialize)]
/// Driver represents a volume driver.
pub struct Driver {
    #[serde(rename = "Name")]
    pub name: Option<String>,
    #[serde(rename = "Options")]
    pub options: Option<std::collections::HashMap<String, String>>,
}

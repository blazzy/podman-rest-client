use serde::{Deserialize, Serialize};
#[derive(Default, Debug, Serialize, Deserialize)]
pub struct SecretDriverSpec {
    #[serde(rename = "Name")]
    pub name: Option<String>,
    #[serde(rename = "Options")]
    pub options: Option<std::collections::HashMap<String, Option<String>>>,
}

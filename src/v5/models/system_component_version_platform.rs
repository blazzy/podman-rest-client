use serde::{Deserialize, Serialize};
#[derive(Default, Debug, Serialize, Deserialize)]
pub struct SystemComponentVersionPlatform {
    #[serde(rename = "Name")]
    pub name: Option<String>,
}

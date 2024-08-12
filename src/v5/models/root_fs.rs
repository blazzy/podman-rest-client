use serde::{Deserialize, Serialize};
#[derive(Default, Debug, Serialize, Deserialize)]
/// RootFS holds the root fs information of an image.
pub struct RootFs {
    #[serde(rename = "Layers")]
    pub layers: Option<Vec<String>>,
    #[serde(rename = "Type")]
    pub r#type: Option<String>,
}

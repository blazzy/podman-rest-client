use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Serialize, Deserialize)]
/// DeleteResponse delete response
pub struct DeleteResponse {
    /// The image ID of an image that was deleted
    #[serde(rename = "Deleted")]
    pub deleted: Option<String>,

    /// The image ID of an image that was untagged
    #[serde(rename = "Untagged")]
    pub untagged: Option<String>,
}

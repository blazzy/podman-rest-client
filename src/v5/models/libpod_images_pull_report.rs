use serde::{Deserialize, Serialize};
#[derive(Default, Debug, Serialize, Deserialize)]
pub struct LibpodImagesPullReport {
    /// Error contains text of errors from c/image
    pub error: Option<String>,
    /// ID contains image id (retained for backwards compatibility)
    pub id: Option<String>,
    /// Images contains the ID's of the images pulled
    pub images: Option<Vec<String>>,
    /// Stream used to provide output from c/image
    pub stream: Option<String>,
}

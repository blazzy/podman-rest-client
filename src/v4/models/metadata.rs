use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Serialize, Deserialize)]
/// Metadata contains engine-local data about the image.
pub struct Metadata {
    /// LastTagTime is the date and time at which the image was last tagged.
    #[serde(rename = "LastTagTime")]
    pub last_tag_time: Option<String>,
}

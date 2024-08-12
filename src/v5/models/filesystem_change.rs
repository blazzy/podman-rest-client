use serde::{Deserialize, Serialize};
#[derive(Default, Debug, Serialize, Deserialize)]
/// FilesystemChange Change in the container's filesystem.
pub struct FilesystemChange {
    #[serde(rename = "Kind")]
    pub kind: u8,
    /// Path to file or directory that has changed.
    #[serde(rename = "Path")]
    pub path: String,
}

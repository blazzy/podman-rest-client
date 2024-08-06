use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Serialize, Deserialize)]
/// GraphDriverData Information about the storage driver used to store the container's and
/// image's filesystem.
pub struct GraphDriverData {
    /// Low-level storage metadata, provided as key/value pairs.
    ///
    /// This information is driver-specific, and depends on the storage-driver
    /// in use, and should be used for informational purposes only.
    #[serde(rename = "Data")]
    pub data: std::collections::HashMap<String, String>,

    /// Name of the storage driver.
    #[serde(rename = "Name")]
    pub name: String,
}

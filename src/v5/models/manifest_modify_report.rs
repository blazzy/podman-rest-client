use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Serialize, Deserialize)]
pub struct ManifestModifyReport {
    /// Manifest List ID
    #[serde(rename = "Id")]
    pub id: Option<String>,

    /// Errors associated with operation
    pub errors: Option<Vec<String>>,

    /// Files added to manifest list, otherwise not provided.
    pub files: Option<Vec<String>>,

    /// Images added to or removed from manifest list, otherwise not provided.
    pub images: Option<Vec<String>>,
}

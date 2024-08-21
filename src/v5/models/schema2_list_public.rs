use serde::{Deserialize, Serialize};
#[derive(Default, Debug, Serialize, Deserialize)]
/// Schema2ListPublic is a list of platform-specific manifests.
/// This is publicly visible as c/image/manifest.Schema2List.
/// Internal users should usually use Schema2List instead.
pub struct Schema2ListPublic {
    pub manifests: Option<Vec<crate::v5::models::Schema2ManifestDescriptor>>,
    #[serde(rename = "mediaType")]
    pub media_type: Option<String>,
    #[serde(rename = "schemaVersion")]
    pub schema_version: Option<i64>,
}

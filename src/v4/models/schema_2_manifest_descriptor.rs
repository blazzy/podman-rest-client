use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Serialize, Deserialize)]
/// Schema2ManifestDescriptor references a platform-specific manifest.
/// This is publicly visible as c/image/manifest.Schema2ManifestDescriptor.
pub struct Schema2ManifestDescriptor {
    pub digest: Option<String>,

    #[serde(rename = "mediaType")]
    pub media_type: Option<String>,

    pub platform: Option<super::super::models::Schema2PlatformSpec>,

    pub size: Option<i64>,

    pub urls: Option<Vec<String>>,
}

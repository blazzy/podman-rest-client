use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Serialize, Deserialize)]
/// Schema2PlatformSpec describes the platform which a particular manifest is
/// specialized for.
/// This is publicly visible as c/image/manifest.Schema2PlatformSpec.
pub struct Schema2PlatformSpec {
    pub architecture: Option<String>,

    pub features: Option<Vec<String>>,

    pub os: Option<String>,

    #[serde(rename = "os.features")]
    pub os_features: Option<Vec<String>>,

    #[serde(rename = "os.version")]
    pub os_version: Option<String>,

    pub variant: Option<String>,
}

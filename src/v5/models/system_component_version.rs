use serde::{Deserialize, Serialize};
#[derive(Default, Debug, Serialize, Deserialize)]
/// SystemComponentVersion is the type used by pkg/domain/entities
pub struct SystemComponentVersion {
    #[serde(rename = "ApiVersion")]
    pub api_version: Option<String>,
    #[serde(rename = "Arch")]
    pub arch: Option<String>,
    #[serde(rename = "BuildTime")]
    pub build_time: Option<String>,
    #[serde(rename = "Components")]
    pub components: Option<Vec<crate::v5::models::ComponentVersion>>,
    #[serde(rename = "Experimental")]
    pub experimental: Option<bool>,
    #[serde(rename = "GitCommit")]
    pub git_commit: Option<String>,
    #[serde(rename = "GoVersion")]
    pub go_version: Option<String>,
    #[serde(rename = "KernelVersion")]
    pub kernel_version: Option<String>,
    #[serde(rename = "MinAPIVersion")]
    pub min_api_version: Option<String>,
    #[serde(rename = "Os")]
    pub os: Option<String>,
    #[serde(rename = "Platform")]
    pub platform: Option<crate::v5::models::SystemComponentVersionPlatform>,
    #[serde(rename = "Version")]
    pub version: Option<String>,
}

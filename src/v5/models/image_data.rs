use serde::{Deserialize, Serialize};
#[derive(Default, Debug, Serialize, Deserialize)]
/// ImageData holds the inspect information of an image.
pub struct ImageData {
    #[serde(rename = "Annotations")]
    pub annotations: Option<std::collections::HashMap<String, Option<String>>>,
    #[serde(rename = "Architecture")]
    pub architecture: Option<String>,
    #[serde(rename = "Author")]
    pub author: Option<String>,
    #[serde(rename = "Comment")]
    pub comment: Option<String>,
    #[serde(rename = "Config")]
    pub config: Option<crate::v5::models::ImageConfig>,
    #[serde(rename = "Created")]
    pub created: Option<String>,
    #[serde(rename = "Digest")]
    pub digest: Option<String>,
    #[serde(rename = "GraphDriver")]
    pub graph_driver: Option<crate::v5::models::DriverData>,
    #[serde(rename = "Healthcheck")]
    pub healthcheck: Option<crate::v5::models::Schema2HealthConfig>,
    #[serde(rename = "History")]
    pub history: Option<Vec<crate::v5::models::History>>,
    #[serde(rename = "Id")]
    pub id: Option<String>,
    #[serde(rename = "Labels")]
    pub labels: Option<std::collections::HashMap<String, Option<String>>>,
    #[serde(rename = "ManifestType")]
    pub manifest_type: Option<String>,
    #[serde(rename = "NamesHistory")]
    pub names_history: Option<Vec<String>>,
    #[serde(rename = "Os")]
    pub os: Option<String>,
    #[serde(rename = "Parent")]
    pub parent: Option<String>,
    #[serde(rename = "RepoDigests")]
    pub repo_digests: Option<Vec<String>>,
    #[serde(rename = "RepoTags")]
    pub repo_tags: Option<Vec<String>>,
    #[serde(rename = "RootFS")]
    pub root_fs: Option<crate::v5::models::RootFs>,
    #[serde(rename = "Size")]
    pub size: Option<i64>,
    #[serde(rename = "User")]
    pub user: Option<String>,
    #[serde(rename = "Version")]
    pub version: Option<String>,
    #[serde(rename = "VirtualSize")]
    pub virtual_size: Option<i64>,
}

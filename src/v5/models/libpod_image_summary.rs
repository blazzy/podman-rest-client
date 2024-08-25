use serde::{Deserialize, Serialize};
#[derive(Default, Debug, Serialize, Deserialize)]
pub struct LibpodImageSummary {
    /// Podman extensions
    #[serde(rename = "Arch")]
    pub arch: Option<String>,
    #[serde(rename = "Containers")]
    pub containers: Option<i64>,
    #[serde(rename = "Created")]
    pub created: Option<i64>,
    #[serde(rename = "Dangling")]
    pub dangling: Option<bool>,
    #[serde(rename = "Digest")]
    pub digest: Option<String>,
    #[serde(rename = "History")]
    pub history: Option<Vec<String>>,
    #[serde(rename = "Id")]
    pub id: Option<String>,
    /// IsManifestList is a ptr so we can distinguish between a true
    /// json empty response and false.  the docker compat side needs to return
    /// empty; where as the libpod side needs a value of true or false
    #[serde(rename = "IsManifestList")]
    pub is_manifest_list: Option<bool>,
    #[serde(rename = "Labels")]
    pub labels: Option<std::collections::HashMap<String, Option<String>>>,
    #[serde(rename = "Names")]
    pub names: Option<Vec<String>>,
    #[serde(rename = "Os")]
    pub os: Option<String>,
    #[serde(rename = "ParentId")]
    pub parent_id: Option<String>,
    #[serde(rename = "ReadOnly")]
    pub read_only: Option<bool>,
    #[serde(rename = "RepoDigests")]
    pub repo_digests: Option<Vec<String>>,
    #[serde(rename = "RepoTags")]
    pub repo_tags: Option<Vec<String>>,
    #[serde(rename = "SharedSize")]
    pub shared_size: Option<i64>,
    #[serde(rename = "Size")]
    pub size: Option<i64>,
    #[serde(rename = "VirtualSize")]
    pub virtual_size: Option<i64>,
}

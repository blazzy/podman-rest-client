use serde::{Deserialize, Serialize};
#[derive(Default, Debug, Serialize, Deserialize)]
pub struct ListPodsReport {
    #[serde(rename = "Cgroup")]
    pub cgroup: Option<String>,
    #[serde(rename = "Containers")]
    pub containers: Option<Vec<crate::v5::models::ListPodContainer>>,
    #[serde(rename = "Created")]
    pub created: Option<String>,
    #[serde(rename = "Id")]
    pub id: Option<String>,
    #[serde(rename = "InfraId")]
    pub infra_id: Option<String>,
    #[serde(rename = "Labels")]
    pub labels: Option<std::collections::HashMap<String, Option<String>>>,
    #[serde(rename = "Name")]
    pub name: Option<String>,
    #[serde(rename = "Namespace")]
    pub namespace: Option<String>,
    /// Network names connected to infra container
    #[serde(rename = "Networks")]
    pub networks: Option<Vec<String>>,
    #[serde(rename = "Status")]
    pub status: Option<String>,
}

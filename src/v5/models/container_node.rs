use serde::{Deserialize, Serialize};
#[derive(Default, Debug, Serialize, Deserialize)]
/// ContainerNode stores information about the node that a container
/// is running on.  It's only used by the Docker Swarm standalone API
pub struct ContainerNode {
    #[serde(rename = "Addr")]
    pub addr: Option<String>,
    #[serde(rename = "Cpus")]
    pub cpus: Option<i64>,
    #[serde(rename = "ID")]
    pub id: Option<String>,
    #[serde(rename = "IP")]
    pub ip: Option<String>,
    #[serde(rename = "Labels")]
    pub labels: Option<std::collections::HashMap<String, Option<String>>>,
    #[serde(rename = "Memory")]
    pub memory: Option<i64>,
    #[serde(rename = "Name")]
    pub name: Option<String>,
}

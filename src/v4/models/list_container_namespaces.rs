use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Serialize, Deserialize)]
/// ListContainerNamespaces contains the identifiers of the container's Linux namespaces
pub struct ListContainerNamespaces {
    /// Cgroup namespace
    #[serde(rename = "Cgroup")]
    pub cgroup: Option<String>,

    /// IPC namespace
    #[serde(rename = "Ipc")]
    pub ipc: Option<String>,

    /// Mount namespace
    #[serde(rename = "Mnt")]
    pub mnt: Option<String>,

    /// Network namespace
    #[serde(rename = "Net")]
    pub net: Option<String>,

    /// PID namespace
    #[serde(rename = "Pidns")]
    pub pidns: Option<String>,

    /// User namespace
    #[serde(rename = "User")]
    pub user: Option<String>,

    /// UTS namespace
    #[serde(rename = "Uts")]
    pub uts: Option<String>,
}

use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Serialize, Deserialize)]
/// LinuxIDMapping specifies UID/GID mappings
pub struct LinuxIdMapping {
    /// ContainerID is the starting UID/GID in the container
    #[serde(rename = "containerID")]
    pub container_id: Option<u32>,

    /// HostID is the starting UID/GID on the host to be mapped to 'ContainerID'
    #[serde(rename = "hostID")]
    pub host_id: Option<u32>,

    /// Size is the number of IDs to be mapped
    pub size: Option<u32>,
}

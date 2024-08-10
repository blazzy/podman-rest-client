use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Serialize, Deserialize)]
/// DeviceMapping represents the device mapping between the host and the container.
pub struct DeviceMapping {
    #[serde(rename = "CgroupPermissions")]
    pub cgroup_permissions: Option<String>,

    #[serde(rename = "PathInContainer")]
    pub path_in_container: Option<String>,

    #[serde(rename = "PathOnHost")]
    pub path_on_host: Option<String>,
}

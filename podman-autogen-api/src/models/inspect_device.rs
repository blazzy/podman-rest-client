use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Serialize, Deserialize)]
/// InspectDevice is a single device that will be mounted into the container.
pub struct InspectDevice {
    /// CgroupPermissions is the permissions of the mounted device.
    /// Presently not populated.
    /// TODO.
    #[serde(rename = "CgroupPermissions")]
    pub cgroup_permissions: Option<String>,

    /// PathInContainer is the path of the device within the container.
    #[serde(rename = "PathInContainer")]
    pub path_in_container: Option<String>,

    /// PathOnHost is the path of the device on the host.
    #[serde(rename = "PathOnHost")]
    pub path_on_host: Option<String>,
}

use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Serialize, Deserialize)]
/// ContainerCgroupConfig contains configuration information about a container's
/// cgroups.
pub struct ContainerCgroupConfig {
    /// CgroupParent is the container's Cgroup parent.
    /// If not set, the default for the current cgroup driver will be used.
    /// Optional.
    pub cgroup_parent: Option<String>,

    pub cgroupns: Option<super::super::models::Namespace>,

    /// CgroupsMode sets a policy for how cgroups will be created for the
    /// container, including the ability to disable creation entirely.
    /// Optional.
    pub cgroups_mode: Option<String>,
}

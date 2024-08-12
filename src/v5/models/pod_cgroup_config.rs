use serde::{Deserialize, Serialize};
#[derive(Default, Debug, Serialize, Deserialize)]
/// PodCgroupConfig contains configuration options about a pod's cgroups.
/// This will be expanded in future updates to pods.
pub struct PodCgroupConfig {
    /// CgroupParent is the parent for the Cgroup that the pod will create.
    /// This pod cgroup will, in turn, be the default cgroup parent for all
    /// containers in the pod.
    /// Optional.
    pub cgroup_parent: Option<String>,
}

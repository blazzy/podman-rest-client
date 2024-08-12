use serde::{Deserialize, Serialize};
#[derive(Default, Debug, Serialize, Deserialize)]
/// BindOptions defines options specific to mounts of type "bind".
pub struct BindOptions {
    #[serde(rename = "CreateMountpoint")]
    pub create_mountpoint: Option<bool>,
    #[serde(rename = "NonRecursive")]
    pub non_recursive: Option<bool>,
    #[serde(rename = "Propagation")]
    pub propagation: Option<String>,
    /// ReadOnlyForceRecursive raises an error if the mount cannot be made recursively read-only.
    #[serde(rename = "ReadOnlyForceRecursive")]
    pub read_only_force_recursive: Option<bool>,
    /// ReadOnlyNonRecursive makes the mount non-recursively read-only, but still leaves the mount recursive
    /// (unless NonRecursive is set to true in conjunction).
    #[serde(rename = "ReadOnlyNonRecursive")]
    pub read_only_non_recursive: Option<bool>,
}

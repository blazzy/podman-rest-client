use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Serialize, Deserialize)]
/// TypeMount contains options for using a volume as a Mount-type
/// volume.
pub struct TypeMount {
    /// FsType specifies the filesystem type for the mount volume. Optional.
    #[serde(rename = "FsType")]
    pub fs_type: Option<String>,

    /// MountFlags defines flags to pass when mounting the volume. Optional.
    #[serde(rename = "MountFlags")]
    pub mount_flags: Option<Vec<String>>,
}

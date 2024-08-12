use serde::{Deserialize, Serialize};
#[derive(Default, Debug, Serialize, Deserialize)]
/// MountPoint represents a mount point configuration inside the container.
/// This is used for reporting the mountpoints in use by a container.
pub struct MountPoint {
    /// Destination is the path relative to the container root (`/`) where the
    /// Source is mounted inside the container.
    #[serde(rename = "Destination")]
    pub destination: Option<String>,
    /// Driver is the volume driver used to create the volume (if it is a volume).
    #[serde(rename = "Driver")]
    pub driver: Option<String>,
    /// Mode is a comma separated list of options supplied by the user when
    /// creating the bind/volume mount.
    ///
    /// The default is platform-specific (`"z"` on Linux, empty on Windows).
    #[serde(rename = "Mode")]
    pub mode: Option<String>,
    /// Name is the name reference to the underlying data defined by `Source`
    /// e.g., the volume name.
    #[serde(rename = "Name")]
    pub name: Option<String>,
    #[serde(rename = "Propagation")]
    pub propagation: Option<String>,
    /// RW indicates whether the mount is mounted writable (read-write).
    #[serde(rename = "RW")]
    pub rw: Option<bool>,
    /// Source is the source location of the mount.
    ///
    /// For volumes, this contains the storage location of the volume (within
    /// `/var/lib/docker/volumes/`). For bind-mounts, and `npipe`, this contains
    /// the source (host) part of the bind-mount. For `tmpfs` mount points, this
    /// field is empty.
    #[serde(rename = "Source")]
    pub source: Option<String>,
    #[serde(rename = "Type")]
    pub r#type: Option<String>,
}

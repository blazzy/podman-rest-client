use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Serialize, Deserialize)]
pub struct VolumeConfigResponse {
    /// Anonymous indicates that the volume was created as an anonymous
    /// volume for a specific container, and will be removed when any
    /// container using it is removed.
    #[serde(rename = "Anonymous")]
    pub anonymous: Option<bool>,

    /// CreatedAt is the date and time the volume was created at. This is not
    /// stored for older Libpod volumes; if so, it will be omitted.
    #[serde(rename = "CreatedAt")]
    pub created_at: Option<String>,

    /// Driver is the driver used to create the volume.
    /// If set to "local" or "", the Local driver (Podman built-in code) is
    /// used to service the volume; otherwise, a volume plugin with the given
    /// name is used to mount and manage the volume.
    #[serde(rename = "Driver")]
    pub driver: Option<String>,

    /// GID is the GID that the volume was created with.
    #[serde(rename = "GID")]
    pub gid: Option<i64>,

    /// Labels includes the volume's configured labels, key:value pairs that
    /// can be passed during volume creation to provide information for third
    /// party tools.
    #[serde(rename = "Labels")]
    pub labels: Option<std::collections::HashMap<String, String>>,

    /// LockNumber is the number of the volume's Libpod lock.
    #[serde(rename = "LockNumber")]
    pub lock_number: Option<u32>,

    /// MountCount is the number of times this volume has been mounted.
    #[serde(rename = "MountCount")]
    pub mount_count: Option<u64>,

    /// Mountpoint is the path on the host where the volume is mounted.
    #[serde(rename = "Mountpoint")]
    pub mountpoint: Option<String>,

    /// Name is the name of the volume.
    #[serde(rename = "Name")]
    pub name: Option<String>,

    /// NeedsChown indicates that the next time the volume is mounted into
    /// a container, the container will chown the volume to the container process
    /// UID/GID.
    #[serde(rename = "NeedsChown")]
    pub needs_chown: Option<bool>,

    /// NeedsCopyUp indicates that the next time the volume is mounted into
    #[serde(rename = "NeedsCopyUp")]
    pub needs_copy_up: Option<bool>,

    /// Options is a set of options that were used when creating the volume.
    /// For the Local driver, these are mount options that will be used to
    /// determine how a local filesystem is mounted; they are handled as
    /// parameters to Mount in a manner described in the volume create
    /// manpage.
    /// For non-local drivers, these are passed as-is to the volume plugin.
    #[serde(rename = "Options")]
    pub options: Option<std::collections::HashMap<String, String>>,

    /// Scope is unused and provided solely for Docker compatibility. It is
    /// unconditionally set to "local".
    #[serde(rename = "Scope")]
    pub scope: Option<String>,

    /// Status is used to return information on the volume's current state,
    /// if the volume was created using a volume plugin (uses a Driver that
    /// is not the local driver).
    /// Status is provided to us by an external program, so no guarantees are
    /// made about its format or contents. Further, it is an optional field,
    /// so it may not be set even in cases where a volume plugin is in use.
    #[serde(rename = "Status")]
    pub status: Option<std::collections::HashMap<String, ()>>,

    /// StorageID is the ID of the container backing the volume in c/storage.
    /// Only used with Image Volumes.
    #[serde(rename = "StorageID")]
    pub storage_id: Option<String>,

    /// Timeout is the specified driver timeout if given
    #[serde(rename = "Timeout")]
    pub timeout: Option<u64>,

    /// UID is the UID that the volume was created with.
    #[serde(rename = "UID")]
    pub uid: Option<i64>,
}

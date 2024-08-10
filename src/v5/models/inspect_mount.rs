use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Serialize, Deserialize)]
/// InspectMount provides a record of a single mount in a container. It contains
/// fields for both named and normal volumes. Only user-specified volumes will be
/// included, and tmpfs volumes are not included even if the user specified them.
pub struct InspectMount {
    /// The destination directory for the volume. Specified as a path within
    /// the container, as it would be passed into the OCI runtime.
    #[serde(rename = "Destination")]
    pub destination: Option<String>,

    /// The driver used for the named volume. Empty for bind mounts.
    #[serde(rename = "Driver")]
    pub driver: Option<String>,

    /// Contains SELinux :z/:Z mount options. Unclear what, if anything, else
    /// goes in here.
    #[serde(rename = "Mode")]
    pub mode: Option<String>,

    /// The name of the volume. Empty for bind mounts.
    #[serde(rename = "Name")]
    pub name: Option<String>,

    /// All remaining mount options. Additional data, not present in the
    /// original output.
    #[serde(rename = "Options")]
    pub options: Option<Vec<String>>,

    /// Mount propagation for the mount. Can be empty if not specified, but
    /// is always printed - no omitempty.
    #[serde(rename = "Propagation")]
    pub propagation: Option<String>,

    /// Whether the volume is read-write
    #[serde(rename = "RW")]
    pub rw: Option<bool>,

    /// The source directory for the volume.
    #[serde(rename = "Source")]
    pub source: Option<String>,

    /// Whether the mount is a volume or bind mount. Allowed values are
    /// "volume" and "bind".
    #[serde(rename = "Type")]
    pub r#type: Option<String>,
}

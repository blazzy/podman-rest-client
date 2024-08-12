use serde::{Deserialize, Serialize};
#[derive(Default, Debug, Serialize, Deserialize)]
/// ImageVolume is a volume based on a container image.  The container image is
/// first mounted on the host and is then bind-mounted into the container.  An
/// ImageVolume is always mounted read-only.
pub struct ImageVolume {
    /// Destination is the absolute path of the mount in the container.
    #[serde(rename = "Destination")]
    pub destination: Option<String>,
    /// ReadWrite sets the volume writable.
    #[serde(rename = "ReadWrite")]
    pub read_write: Option<bool>,
    /// Source is the source of the image volume.  The image can be referred
    /// to by name and by ID.
    #[serde(rename = "Source")]
    pub source: Option<String>,
    /// SubPath mounts a particular path within the image.
    /// If empty, the whole image is mounted.
    #[serde(rename = "subPath")]
    pub sub_path: Option<String>,
}

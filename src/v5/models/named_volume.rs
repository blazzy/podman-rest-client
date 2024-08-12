use serde::{Deserialize, Serialize};
#[derive(Default, Debug, Serialize, Deserialize)]
/// NamedVolume holds information about a named volume that will be mounted into
/// the container.
pub struct NamedVolume {
    /// Destination to mount the named volume within the container. Must be
    /// an absolute path. Path will be created if it does not exist.
    #[serde(rename = "Dest")]
    pub dest: Option<String>,
    /// IsAnonymous sets the named volume as anonymous even if it has a name
    /// This is used for emptyDir volumes from a kube yaml
    #[serde(rename = "IsAnonymous")]
    pub is_anonymous: Option<bool>,
    /// Name is the name of the named volume to be mounted. May be empty.
    /// If empty, a new named volume with a pseudorandomly generated name
    /// will be mounted at the given destination.
    #[serde(rename = "Name")]
    pub name: Option<String>,
    /// Options are options that the named volume will be mounted with.
    #[serde(rename = "Options")]
    pub options: Option<Vec<String>>,
    /// SubPath stores the sub directory of the named volume to be mounted in the container
    #[serde(rename = "SubPath")]
    pub sub_path: Option<String>,
}

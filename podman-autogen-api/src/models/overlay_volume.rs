use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Serialize, Deserialize)]
/// OverlayVolume holds information about an overlay volume that will be mounted into
/// the container.
pub struct OverlayVolume {
    /// Destination is the absolute path where the mount will be placed in the container.
    pub destination: Option<String>,

    /// Options holds overlay volume options.
    pub options: Option<Vec<String>>,

    /// Source specifies the source path of the mount.
    pub source: Option<String>,
}

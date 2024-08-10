use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Serialize, Deserialize)]
/// InspectUlimit is a ulimit that will be applied to the container.
pub struct InspectUlimit {
    /// Hard is the hard limit that will be applied.
    #[serde(rename = "Hard")]
    pub hard: Option<i64>,

    /// Name is the name (type) of the ulimit.
    #[serde(rename = "Name")]
    pub name: Option<String>,

    /// Soft is the soft limit that will be applied.
    #[serde(rename = "Soft")]
    pub soft: Option<i64>,
}

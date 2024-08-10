use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Serialize, Deserialize)]
/// Version represents the internal object version.
pub struct Version {
    #[serde(rename = "Index")]
    pub index: Option<u64>,
}

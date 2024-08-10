use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Serialize, Deserialize)]
/// POSIXRlimit type and restrictions
pub struct PosixRlimit {
    /// Hard is the hard limit for the specified type
    pub hard: Option<u64>,

    /// Soft is the soft limit for the specified type
    pub soft: Option<u64>,

    /// Type of the rlimit to set
    #[serde(rename = "type")]
    pub r#type: Option<String>,
}

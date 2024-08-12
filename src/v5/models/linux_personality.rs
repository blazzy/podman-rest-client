use serde::{Deserialize, Serialize};
#[derive(Default, Debug, Serialize, Deserialize)]
/// LinuxPersonality represents the Linux personality syscall input
pub struct LinuxPersonality {
    pub domain: Option<String>,
    /// Additional flags
    pub flags: Option<Vec<String>>,
}

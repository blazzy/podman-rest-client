use serde::{Deserialize, Serialize};
#[derive(Default, Debug, Serialize, Deserialize)]
/// DistributionInfo describes the host distribution for libpod
pub struct DistributionInfo {
    pub codename: Option<String>,
    pub distribution: Option<String>,
    pub variant: Option<String>,
    pub version: Option<String>,
}

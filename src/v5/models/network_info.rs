use serde::{Deserialize, Serialize};
#[derive(Default, Debug, Serialize, Deserialize)]
/// NetworkInfo contains the network information.
pub struct NetworkInfo {
    pub backend: Option<String>,
    pub dns: Option<crate::v5::models::DnsNetworkInfo>,
    pub package: Option<String>,
    pub path: Option<String>,
    pub version: Option<String>,
}

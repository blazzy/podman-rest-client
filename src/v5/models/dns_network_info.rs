use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Serialize, Deserialize)]
/// NetworkInfo contains the DNS information.
pub struct DnsNetworkInfo {
    pub package: Option<String>,

    pub path: Option<String>,

    pub version: Option<String>,
}

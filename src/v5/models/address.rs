use serde::{Deserialize, Serialize};
#[derive(Default, Debug, Serialize, Deserialize)]
/// Address represents an IP address.
pub struct Address {
    #[serde(rename = "Addr")]
    pub addr: Option<String>,
    #[serde(rename = "PrefixLength")]
    pub prefix_length: Option<i64>,
}

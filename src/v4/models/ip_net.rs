use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Serialize, Deserialize)]
/// An IPNet represents an IP network.
pub struct IpNet {
    #[serde(rename = "IP")]
    pub ip: Option<String>,

    #[serde(rename = "Mask")]
    pub mask: Option<Vec<u8>>,
}

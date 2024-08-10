use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Serialize, Deserialize)]
/// IPAMConfig represents IPAM configurations
pub struct IpamConfig {
    #[serde(rename = "AuxiliaryAddresses")]
    pub auxiliary_addresses: Option<std::collections::HashMap<String, String>>,

    #[serde(rename = "Gateway")]
    pub gateway: Option<String>,

    #[serde(rename = "IPRange")]
    pub ip_range: Option<String>,

    #[serde(rename = "Subnet")]
    pub subnet: Option<String>,
}

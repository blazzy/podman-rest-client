use serde::{Deserialize, Serialize};
#[derive(Default, Debug, Serialize, Deserialize)]
/// LeaseRange contains the range where IP are leased.
pub struct LeaseRange {
    /// EndIP last IP in the subnet which should be used to assign ips.
    pub end_ip: Option<String>,
    /// StartIP first IP in the subnet which should be used to assign ips.
    pub start_ip: Option<String>,
}

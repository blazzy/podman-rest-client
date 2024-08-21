use serde::{Deserialize, Serialize};
#[derive(Default, Debug, Serialize, Deserialize)]
pub struct Subnet {
    /// Gateway IP for this Network.
    pub gateway: Option<String>,
    pub lease_range: Option<crate::v5::models::LeaseRange>,
    /// Subnet for this Network in CIDR form.
    pub subnet: Option<String>,
}

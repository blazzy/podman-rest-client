use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Serialize, Deserialize)]
pub struct Route {
    /// Destination for this route in CIDR form.
    pub destination: Option<String>,

    /// Gateway IP for this route.
    pub gateway: Option<String>,

    /// Metric for this route. Optional.
    pub metric: Option<u32>,
}

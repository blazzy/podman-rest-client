use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Serialize, Deserialize)]
/// CapacityRange describes the minimum and maximum capacity a volume should be
/// created with
pub struct CapacityRange {
    /// LimitBytes specifies that a volume must not be bigger than this. The
    /// value of 0 indicates an unspecified maximum
    #[serde(rename = "LimitBytes")]
    pub limit_bytes: Option<i64>,

    /// RequiredBytes specifies that a volume must be at least this big. The
    /// value of 0 indicates an unspecified minimum.
    #[serde(rename = "RequiredBytes")]
    pub required_bytes: Option<i64>,
}

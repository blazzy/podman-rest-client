use serde::{Deserialize, Serialize};
#[derive(Default, Debug, Serialize, Deserialize)]
/// UsageData Usage details about the volume. This information is used by the
/// `GET /system/df` endpoint, and omitted in other endpoints.
pub struct UsageData {
    /// The number of containers referencing this volume. This field
    /// is set to `-1` if the reference-count is not available.
    #[serde(rename = "RefCount")]
    pub ref_count: i64,
    /// Amount of disk space used by the volume (in bytes). This information
    /// is only available for volumes created with the `"local"` volume
    /// driver. For volumes created with other volume drivers, this field
    /// is set to `-1` ("not available")
    #[serde(rename = "Size")]
    pub size: i64,
}

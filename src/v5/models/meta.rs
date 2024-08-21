use serde::{Deserialize, Serialize};
#[derive(Default, Debug, Serialize, Deserialize)]
/// Meta is a base object inherited by most of the other once.
pub struct Meta {
    #[serde(rename = "CreatedAt")]
    pub created_at: Option<String>,
    #[serde(rename = "UpdatedAt")]
    pub updated_at: Option<String>,
    #[serde(rename = "Version")]
    pub version: Option<crate::v5::models::Version>,
}

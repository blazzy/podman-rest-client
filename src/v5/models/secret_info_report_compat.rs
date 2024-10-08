use serde::{Deserialize, Serialize};
#[derive(Default, Debug, Serialize, Deserialize)]
pub struct SecretInfoReportCompat {
    #[serde(rename = "CreatedAt")]
    pub created_at: Option<String>,
    #[serde(rename = "ID")]
    pub id: Option<String>,
    #[serde(rename = "SecretData")]
    pub secret_data: Option<String>,
    #[serde(rename = "Spec")]
    pub spec: Option<crate::v5::models::SecretSpec>,
    #[serde(rename = "UpdatedAt")]
    pub updated_at: Option<String>,
    #[serde(rename = "Version")]
    pub version: Option<crate::v5::models::SecretVersion>,
}

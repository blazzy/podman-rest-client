use serde::{Deserialize, Serialize};
#[derive(Default, Debug, Serialize, Deserialize)]
pub struct SecretCreate {
    /// Base64-url-safe-encoded (RFC 4648) data to store as secret.
    #[serde(rename = "Data")]
    pub data: Option<String>,
    #[serde(rename = "Driver")]
    pub driver: Option<super::super::models::SecretDriverSpec>,
    /// Labels are labels on the secret
    #[serde(rename = "Labels")]
    pub labels: Option<std::collections::HashMap<String, String>>,
    /// User-defined name of the secret.
    #[serde(rename = "Name")]
    pub name: Option<String>,
}

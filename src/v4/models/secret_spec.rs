use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Serialize, Deserialize)]
pub struct SecretSpec {
    #[serde(rename = "Driver")]
    pub driver: Option<super::super::models::SecretDriverSpec>,

    #[serde(rename = "Labels")]
    pub labels: Option<std::collections::HashMap<String, String>>,

    #[serde(rename = "Name")]
    pub name: Option<String>,
}
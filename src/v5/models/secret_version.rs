use serde::{Deserialize, Serialize};
#[derive(Default, Debug, Serialize, Deserialize)]
pub struct SecretVersion {
    #[serde(rename = "Index")]
    pub index: Option<i64>,
}

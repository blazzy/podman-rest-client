use serde::{Deserialize, Serialize};
#[derive(Default, Debug, Serialize, Deserialize)]
pub struct PlaySecret {
    #[serde(rename = "CreateReport")]
    pub create_report: Option<super::super::models::SecretCreateReport>,
}

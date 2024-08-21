use serde::{Deserialize, Serialize};
#[derive(Default, Debug, Serialize, Deserialize)]
pub struct PlaySecret {
    #[serde(rename = "CreateReport")]
    pub create_report: Option<crate::v5::models::SecretCreateReport>,
}

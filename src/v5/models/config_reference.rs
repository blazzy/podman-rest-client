use serde::{Deserialize, Serialize};
#[derive(Default, Debug, Serialize, Deserialize)]
/// ConfigReference specifies the source which provides a network's configuration
pub struct ConfigReference {
    #[serde(rename = "Network")]
    pub network: Option<String>,
}

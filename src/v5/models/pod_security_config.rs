use serde::{Deserialize, Serialize};
#[derive(Default, Debug, Serialize, Deserialize)]
pub struct PodSecurityConfig {
    pub idmappings: Option<crate::v5::models::IdMappingOptions>,
    pub security_opt: Option<Vec<String>>,
}

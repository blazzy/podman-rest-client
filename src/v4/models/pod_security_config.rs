use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Serialize, Deserialize)]
pub struct PodSecurityConfig {
    pub idmappings: Option<super::super::models::IdMappingOptions>,

    pub security_opt: Option<Vec<String>>,
}

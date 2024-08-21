use serde::{Deserialize, Serialize};
#[derive(Default, Debug, Serialize, Deserialize)]
/// IDMappings describe the GID and UID mappings
pub struct IdMappings {
    pub gidmap: Option<Vec<crate::v5::models::IdMap>>,
    pub uidmap: Option<Vec<crate::v5::models::IdMap>>,
}

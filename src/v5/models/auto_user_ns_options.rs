use serde::{Deserialize, Serialize};
#[derive(Default, Debug, Serialize, Deserialize)]
/// AutoUserNsOptions defines how to automatically create a user namespace.
pub struct AutoUserNsOptions {
    /// AdditionalGIDMappings specified additional GID mappings to include in
    /// the generated user namespace.
    #[serde(rename = "AdditionalGIDMappings")]
    pub additional_gid_mappings: Option<Vec<super::super::models::IdMap>>,
    /// AdditionalUIDMappings specified additional UID mappings to include in
    /// the generated user namespace.
    #[serde(rename = "AdditionalUIDMappings")]
    pub additional_uid_mappings: Option<Vec<super::super::models::IdMap>>,
    /// GroupFile to use if the container uses a volume.
    #[serde(rename = "GroupFile")]
    pub group_file: Option<String>,
    /// InitialSize defines the minimum size for the user namespace.
    /// The created user namespace will have at least this size.
    #[serde(rename = "InitialSize")]
    pub initial_size: Option<u32>,
    /// PasswdFile to use if the container uses a volume.
    #[serde(rename = "PasswdFile")]
    pub passwd_file: Option<String>,
    /// Size defines the size for the user namespace.  If it is set to a
    /// value bigger than 0, the user namespace will have exactly this size.
    /// If it is not set, some heuristics will be used to find its size.
    #[serde(rename = "Size")]
    pub size: Option<u32>,
}

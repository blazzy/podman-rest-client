use serde::{Deserialize, Serialize};
#[derive(Default, Debug, Serialize, Deserialize)]
/// IDMappingOptions are used for specifying how ID mapping should be set up for
/// a layer or container.
pub struct IdMappingOptions {
    #[serde(rename = "AutoUserNs")]
    pub auto_user_ns: Option<bool>,
    #[serde(rename = "AutoUserNsOpts")]
    pub auto_user_ns_opts: Option<crate::v5::models::AutoUserNsOptions>,
    #[serde(rename = "GIDMap")]
    pub gid_map: Option<Vec<crate::v5::models::IdMap>>,
    #[serde(rename = "HostGIDMapping")]
    pub host_gid_mapping: Option<bool>,
    /// UIDMap and GIDMap are used for setting up a layer's root filesystem
    /// for use inside of a user namespace where ID mapping is being used.
    /// If HostUIDMapping/HostGIDMapping is true, no mapping of the
    /// respective type will be used.  Otherwise, if UIDMap and/or GIDMap
    /// contain at least one mapping, one or both will be used.  By default,
    /// if neither of those conditions apply, if the layer has a parent
    /// layer, the parent layer's mapping will be used, and if it does not
    /// have a parent layer, the mapping which was passed to the Store
    /// object when it was initialized will be used.
    #[serde(rename = "HostUIDMapping")]
    pub host_uid_mapping: Option<bool>,
    #[serde(rename = "UIDMap")]
    pub uid_map: Option<Vec<crate::v5::models::IdMap>>,
}

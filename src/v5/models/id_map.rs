use serde::{Deserialize, Serialize};
#[derive(Default, Debug, Serialize, Deserialize)]
/// IDMap contains a single entry for user namespace range remapping. An array
/// of IDMap entries represents the structure that will be provided to the Linux
/// kernel for creating a user namespace.
pub struct IdMap {
    pub container_id: Option<i64>,
    pub host_id: Option<i64>,
    pub size: Option<i64>,
}

use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Serialize, Deserialize)]
/// ImageStore describes the image store.  Right now only the number
/// of images present
pub struct ImageStore {
    pub number: Option<i64>,
}

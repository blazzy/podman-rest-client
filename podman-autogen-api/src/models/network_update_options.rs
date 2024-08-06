use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Serialize, Deserialize)]
/// NetworkUpdateOptions describes options to update a network
pub struct NetworkUpdateOptions {
    pub adddnsservers: Option<Vec<String>>,

    pub removednsservers: Option<Vec<String>>,
}

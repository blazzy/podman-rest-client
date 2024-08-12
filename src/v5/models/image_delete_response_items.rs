use serde::{Deserialize, Serialize};
#[derive(Default, Debug, Serialize, Deserialize)]
pub struct ImageDeleteResponseItems {
    pub deleted: Option<String>,
    pub untagged: Option<Vec<String>>,
}

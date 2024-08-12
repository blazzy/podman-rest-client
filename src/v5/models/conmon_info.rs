use serde::{Deserialize, Serialize};
#[derive(Default, Debug, Serialize, Deserialize)]
/// ConmonInfo describes the conmon executable being used
pub struct ConmonInfo {
    pub package: Option<String>,
    pub path: Option<String>,
    pub version: Option<String>,
}

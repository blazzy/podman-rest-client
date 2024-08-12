use serde::{Deserialize, Serialize};
#[derive(Default, Debug, Serialize, Deserialize)]
/// PastaInfo describes the pasta executable that is being used
pub struct PastaInfo {
    pub executable: Option<String>,
    pub package: Option<String>,
    pub version: Option<String>,
}

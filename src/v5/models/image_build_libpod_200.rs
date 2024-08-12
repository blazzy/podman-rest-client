use serde::{Deserialize, Serialize};
#[derive(Default, Debug, Serialize, Deserialize)]
pub struct ImageBuildLibpod200 {
    /// output from build process
    pub stream: String,
}

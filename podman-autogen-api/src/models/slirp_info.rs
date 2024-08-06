use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Serialize, Deserialize)]
/// SlirpInfo describes the slirp executable that is being used
pub struct SlirpInfo {
    pub executable: Option<String>,

    pub package: Option<String>,

    pub version: Option<String>,
}

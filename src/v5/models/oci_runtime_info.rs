use serde::{Deserialize, Serialize};
#[derive(Default, Debug, Serialize, Deserialize)]
/// OCIRuntimeInfo describes the runtime (crun or runc) being
/// used with podman
pub struct OciRuntimeInfo {
    pub name: Option<String>,
    pub package: Option<String>,
    pub path: Option<String>,
    pub version: Option<String>,
}

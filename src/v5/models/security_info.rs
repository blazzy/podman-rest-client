use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Serialize, Deserialize)]
/// SecurityInfo describes the libpod host
pub struct SecurityInfo {
    #[serde(rename = "apparmorEnabled")]
    pub apparmor_enabled: Option<bool>,

    pub capabilities: Option<String>,

    pub rootless: Option<bool>,

    #[serde(rename = "seccompEnabled")]
    pub seccomp_enabled: Option<bool>,

    #[serde(rename = "seccompProfilePath")]
    pub seccomp_profile_path: Option<String>,

    #[serde(rename = "selinuxEnabled")]
    pub selinux_enabled: Option<bool>,
}

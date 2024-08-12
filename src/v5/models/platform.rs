use serde::{Deserialize, Serialize};
#[derive(Default, Debug, Serialize, Deserialize)]
/// Platform describes the platform which the image in the manifest runs on.
pub struct Platform {
    /// Architecture field specifies the CPU architecture, for example
    /// `amd64` or `ppc64le`.
    pub architecture: Option<String>,
    /// OS specifies the operating system, for example `linux` or `windows`.
    pub os: Option<String>,
    /// OSFeatures is an optional field specifying an array of strings,
    /// each listing a required OS feature (for example on Windows `win32k`).
    #[serde(rename = "os.features")]
    pub os_features: Option<Vec<String>>,
    /// OSVersion is an optional field specifying the operating system
    /// version, for example on Windows `10.0.14393.1066`.
    #[serde(rename = "os.version")]
    pub os_version: Option<String>,
    /// Variant is an optional field specifying a variant of the CPU, for
    /// example `v7` to specify ARMv7 when architecture is `arm`.
    pub variant: Option<String>,
}

use serde::{Deserialize, Serialize};
#[derive(Default, Debug, Serialize, Deserialize)]
/// TmpfsOptions defines options specific to mounts of type "tmpfs".
pub struct TmpfsOptions {
    #[serde(rename = "Mode")]
    pub mode: Option<u32>,
    /// Size sets the size of the tmpfs, in bytes.
    ///
    /// This will be converted to an operating system specific value
    /// depending on the host. For example, on linux, it will be converted to
    /// use a 'k', 'm' or 'g' syntax. BSD, though not widely supported with
    /// docker, uses a straight byte value.
    ///
    /// Percentages are not supported.
    #[serde(rename = "SizeBytes")]
    pub size_bytes: Option<i64>,
}

use serde::{Deserialize, Serialize};
#[derive(Default, Debug, Serialize, Deserialize)]
pub struct ManifestRemoveReport {
    /// Deleted manifest list.
    #[serde(rename = "Deleted")]
    pub deleted: Option<Vec<String>>,
    /// Errors associated with operation
    #[serde(rename = "Errors")]
    pub errors: Option<Vec<String>>,
    /// ExitCode describes the exit codes as described in the `podman rmi`
    /// man page.
    #[serde(rename = "ExitCode")]
    pub exit_code: Option<i64>,
    /// Untagged images. Can be longer than Deleted.
    #[serde(rename = "Untagged")]
    pub untagged: Option<Vec<String>>,
}

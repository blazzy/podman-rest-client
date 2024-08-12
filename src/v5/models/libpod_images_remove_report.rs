use serde::{Deserialize, Serialize};
#[derive(Default, Debug, Serialize, Deserialize)]
/// LibpodImagesRemoveReport is the return type for image removal via the rest
/// api.
pub struct LibpodImagesRemoveReport {
    /// Deleted images.
    #[serde(rename = "Deleted")]
    pub deleted: Option<Vec<String>>,
    /// Image removal requires is to return data and an error.
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

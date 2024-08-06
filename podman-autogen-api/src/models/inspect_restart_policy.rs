use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Serialize, Deserialize)]
/// InspectRestartPolicy holds information about the container's restart policy.
pub struct InspectRestartPolicy {
    /// MaximumRetryCount is the maximum number of retries allowed if the
    /// "on-failure" restart policy is in use. Not used if "on-failure" is
    /// not set.
    #[serde(rename = "MaximumRetryCount")]
    pub maximum_retry_count: Option<u64>,

    /// Name contains the container's restart policy.
    /// Allowable values are "no" or "" (take no action),
    /// "on-failure" (restart on non-zero exit code, with an optional max
    /// retry count), and "always" (always restart on container stop, unless
    /// explicitly requested by API).
    /// Note that this is NOT actually a name of any sort - the poor naming
    /// is for Docker compatibility.
    #[serde(rename = "Name")]
    pub name: Option<String>,
}

use serde::{Deserialize, Serialize};
#[derive(Default, Debug, Serialize, Deserialize)]
/// LogConfig describes the logging characteristics for a container
pub struct LogConfigLibpod {
    /// LogDriver is the container's log driver.
    /// Optional.
    pub driver: Option<String>,
    /// A set of options to accompany the log driver.
    /// Optional.
    pub options: Option<std::collections::HashMap<String, Option<String>>>,
    /// LogPath is the path the container's logs will be stored at.
    /// Only available if LogDriver is set to "json-file" or "k8s-file".
    /// Optional.
    pub path: Option<String>,
    /// Size is the maximum size of the log file
    /// Optional.
    pub size: Option<i64>,
}

#[derive(Default, Debug)]
pub struct ContainerLogs<'a> {
    /// Keep connection after returning logs.
    pub follow: Option<bool>,
    /// Return logs from stdout
    pub stdout: Option<bool>,
    /// Return logs from stderr
    pub stderr: Option<bool>,
    /// Only return logs since this time, as a UNIX timestamp
    pub since: Option<&'a str>,
    /// Only return logs before this time, as a UNIX timestamp
    pub until: Option<&'a str>,
    /// Add timestamps to every log line
    pub timestamps: Option<bool>,
    /// Only return this number of log lines from the end of the logs
    pub tail: Option<&'a str>,
}

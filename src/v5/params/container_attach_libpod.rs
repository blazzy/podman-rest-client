#[derive(Default, Debug)]
pub struct ContainerAttachLibpod<'a> {
    /// keys to use for detaching from the container
    pub detach_keys: Option<&'a str>,
    /// Stream all logs from the container across the connection. Happens before streaming attach (if requested). At least one of logs or stream must be set
    pub logs: Option<bool>,
    /// Attach to the container. If unset, and logs is set, only the container's logs will be sent. At least one of stream or logs must be set
    pub stream: Option<bool>,
    /// Attach to container STDOUT
    pub stdout: Option<bool>,
    /// Attach to container STDERR
    pub stderr: Option<bool>,
    /// Attach to container STDIN
    pub stdin: Option<bool>,
}

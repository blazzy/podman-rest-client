#[derive(Default, Debug)]
pub struct GenerateSystemdLibpod<'a> {
    /// Use container/pod names instead of IDs.
    pub use_name: Option<bool>,
    /// Create a new container instead of starting an existing one.
    pub new: Option<bool>,
    /// Do not generate the header including the Podman version and the timestamp.
    pub no_header: Option<bool>,
    /// Start timeout in seconds.
    pub start_timeout: Option<i64>,
    /// Stop timeout in seconds.
    pub stop_timeout: Option<i64>,
    /// Systemd restart-policy.
    pub restart_policy: Option<&'a str>,
    /// Systemd unit name prefix for containers.
    pub container_prefix: Option<&'a str>,
    /// Systemd unit name prefix for pods.
    pub pod_prefix: Option<&'a str>,
    /// Systemd unit name separator between name/id and prefix.
    pub separator: Option<&'a str>,
    /// Configures the time to sleep before restarting a service.
    pub restart_sec: Option<i64>,
    /// Systemd Wants list for the container or pods.
    pub wants: Option<Vec<&'a str>>,
    /// Systemd After list for the container or pods.
    pub after: Option<Vec<&'a str>>,
    /// Systemd Requires list for the container or pods.
    pub requires: Option<Vec<&'a str>>,
    /// Set environment variables to the systemd unit files.
    pub additional_env_variables: Option<Vec<&'a str>>,
}

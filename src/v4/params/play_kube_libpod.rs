#[derive(Default, Debug)]
pub struct PlayKubeLibpod<'a> {
    /// Logging driver for the containers in the pod.
    pub log_driver: Option<&'a str>,

    /// logging driver options
    pub log_options: Option<Vec<&'a str>>,

    /// USe the network mode or specify an array of networks.
    pub network: Option<Vec<&'a str>>,

    /// do not setup /etc/hosts file in container
    pub no_hosts: Option<bool>,

    /// use annotations that are not truncated to the Kubernetes maximum length of 63 characters
    pub no_trunc: Option<bool>,

    /// publish a container's port, or a range of ports, to the host
    pub publish_ports: Option<Vec<&'a str>>,

    /// replace existing pods and containers
    pub replace: Option<bool>,

    /// Starts a service container before all pods.
    pub service_container: Option<bool>,

    /// Start the pod after creating it.
    pub start: Option<bool>,

    /// Static IPs used for the pods.
    pub static_i_ps: Option<Vec<&'a str>>,

    /// Static MACs used for the pods.
    pub static_ma_cs: Option<Vec<&'a str>>,

    /// Require HTTPS and verify signatures when contacting registries.
    pub tls_verify: Option<bool>,

    /// Set the user namespace mode for the pods.
    pub userns: Option<&'a str>,

    /// Clean up all objects created when a SIGTERM is received or pods exit.
    pub wait: Option<bool>,
}

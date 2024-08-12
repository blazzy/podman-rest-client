#[derive(Default, Debug)]
pub struct ImageBuild<'a> {
    pub content_type: Option<&'a str>,
    pub x_registry_config: Option<&'a str>,
    /// Path within the build context to the `Dockerfile`.
    /// This is ignored if remote is specified and points to an external `Dockerfile`.
    pub dockerfile: Option<&'a str>,
    /// A name and optional tag to apply to the image in the `name:tag` format. If you omit the tag, the default latest value is assumed. You can provide several t parameters.
    pub t: Option<&'a str>,
    /// TBD Extra hosts to add to /etc/hosts
    /// (As of version 1.xx)
    pub extrahosts: Option<&'a str>,
    /// A Git repository URI or HTTP/HTTPS context URI.
    /// If the URI points to a single text file, the file’s contents are placed
    /// into a file called Dockerfile and the image is built from that file. If
    /// the URI points to a tarball, the file is downloaded by the daemon and the
    /// contents therein used as the context for the build. If the URI points to a
    /// tarball and the dockerfile parameter is also specified, there must be a file
    /// with the corresponding path inside the tarball.
    /// (As of version 1.xx)
    pub remote: Option<&'a str>,
    /// Suppress verbose build output
    pub q: Option<bool>,
    /// Do not use the cache when building the image
    /// (As of version 1.xx)
    pub nocache: Option<bool>,
    /// JSON array of images used to build cache resolution
    /// (As of version 1.xx)
    pub cachefrom: Option<&'a str>,
    /// Attempt to pull the image even if an older image exists locally
    /// (As of version 1.xx)
    pub pull: Option<bool>,
    /// Remove intermediate containers after a successful build
    /// (As of version 1.xx)
    pub rm: Option<bool>,
    /// Always remove intermediate containers, even upon failure
    /// (As of version 1.xx)
    pub forcerm: Option<bool>,
    /// Memory is the upper limit (in bytes) on how much memory running containers can use
    /// (As of version 1.xx)
    pub memory: Option<i64>,
    /// MemorySwap limits the amount of memory and swap together
    /// (As of version 1.xx)
    pub memswap: Option<i64>,
    /// CPUShares (relative weight
    /// (As of version 1.xx)
    pub cpushares: Option<i64>,
    /// CPUSetCPUs in which to allow execution (0-3, 0,1)
    /// (As of version 1.xx)
    pub cpusetcpus: Option<&'a str>,
    /// CPUPeriod limits the CPU CFS (Completely Fair Scheduler) period
    /// (As of version 1.xx)
    pub cpuperiod: Option<i64>,
    /// CPUQuota limits the CPU CFS (Completely Fair Scheduler) quota
    /// (As of version 1.xx)
    pub cpuquota: Option<i64>,
    /// JSON map of string pairs denoting build-time variables.
    /// For example, the build argument `Foo` with the value of `bar` would be encoded in JSON as `["Foo":"bar"]`.
    ///
    /// For example, buildargs={"Foo":"bar"}.
    ///
    /// Note(s):
    /// * This should not be used to pass secrets.
    /// * The value of buildargs should be URI component encoded before being passed to the API.
    ///
    /// (As of version 1.xx)
    pub buildargs: Option<&'a str>,
    /// ShmSize is the "size" value to use when mounting an shmfs on the container's /dev/shm directory.
    /// Default is 64MB
    /// (As of version 1.xx)
    pub shmsize: Option<i64>,
    /// Silently ignored.
    /// Squash the resulting images layers into a single layer
    /// (As of version 1.xx)
    pub squash: Option<bool>,
    /// JSON map of key, value pairs to set as labels on the new image
    /// (As of version 1.xx)
    pub labels: Option<&'a str>,
    /// Sets the networking mode for the run commands during build.
    /// Supported standard values are:
    ///   * `bridge` limited to containers within a single host, port mapping required for external access
    ///   * `host` no isolation between host and containers on this network
    ///   * `none` disable all networking for this container
    ///   * container:<nameOrID> share networking with given container
    ///   ---All other values are assumed to be a custom network's name
    /// (As of version 1.xx)
    pub networkmode: Option<&'a str>,
    /// Platform format os[/arch[/variant]]
    /// (As of version 1.xx)
    pub platform: Option<&'a str>,
    /// Target build stage
    /// (As of version 1.xx)
    pub target: Option<&'a str>,
    /// output configuration TBD
    /// (As of version 1.xx)
    pub outputs: Option<&'a str>,
}

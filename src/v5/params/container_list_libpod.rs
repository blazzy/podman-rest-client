#[derive(Default, Debug)]
pub struct ContainerListLibpod<'a> {
    /// Return all containers. By default, only running containers are shown
    pub all: Option<bool>,

    /// Return this number of most recently created containers, including non-running ones.
    pub limit: Option<i64>,

    /// Include namespace information
    pub namespace: Option<bool>,

    /// Ignored. Previously included details on pod name and ID that are currently included by default.
    pub pod: Option<bool>,

    /// Return the size of container as fields SizeRw and SizeRootFs.
    pub size: Option<bool>,

    /// Sync container state with OCI runtime
    pub sync: Option<bool>,

    /// A JSON encoded value of the filters (a `map[string][]string`) to process on the containers list. Available filters:
    /// - `ancestor`=(`<image-name>[:<tag>]`, `<image id>`, or `<image@digest>`)
    /// - `before`=(`<container id>` or `<container name>`)
    /// - `expose`=(`<port>[/<proto>]` or `<startport-endport>/[<proto>]`)
    /// - `exited=<int>` containers with exit code of `<int>`
    /// - `health`=(`starting`, `healthy`, `unhealthy` or `none`)
    /// - `id=<ID>` a container's ID
    /// - `is-task`=(`true` or `false`)
    /// - `label`=(`key` or `"key=value"`) of a container label
    /// - `name=<name>` a container's name
    /// - `network`=(`<network id>` or `<network name>`)
    /// - `pod`=(`<pod id>` or `<pod name>`)
    /// - `publish`=(`<port>[/<proto>]` or `<startport-endport>/[<proto>]`)
    /// - `since`=(`<container id>` or `<container name>`)
    /// - `status`=(`created`, `restarting`, `running`, `removing`, `paused`, `exited` or `dead`)
    /// - `volume`=(`<volume name>` or `<mount point destination>`)
    pub filters: Option<&'a str>,
}

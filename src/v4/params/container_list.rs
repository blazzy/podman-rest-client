#[derive(Default, Debug)]
pub struct ContainerList<'a> {
    /// Return all containers. By default, only running containers are shown
    pub all: Option<bool>,
    /// Return containers in storage not controlled by Podman
    pub external: Option<bool>,
    /// Return this number of most recently created containers, including non-running ones.
    pub limit: Option<i64>,
    /// Return the size of container as fields SizeRw and SizeRootFs.
    pub size: Option<bool>,
    /// Returns a list of containers.
    ///  - ancestor=(<image-name>[:<tag>], <image id>, or <image@digest>)
    ///  - before=(<container id> or <container name>)
    ///  - expose=(<port>[/<proto>]|<startport-endport>/[<proto>])
    ///  - exited=<int> containers with exit code of <int>
    ///  - health=(starting|healthy|unhealthy|none)
    ///  - id=<ID> a container's ID
    ///  - is-task=(true|false)
    ///  - label=key or label="key=value" of a container label
    ///  - name=<name> a container's name
    ///  - network=(<network id> or <network name>)
    ///  - publish=(<port>[/<proto>]|<startport-endport>/[<proto>])
    ///  - since=(<container id> or <container name>)
    ///  - status=(created|restarting|running|removing|paused|exited|dead)
    ///  - volume=(<volume name> or <mount point destination>)
    pub filters: Option<&'a str>,
}

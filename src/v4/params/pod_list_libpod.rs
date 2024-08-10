#[derive(Default, Debug)]
pub struct PodListLibpod<'a> {
    /// JSON encoded value of the filters (a map[string][]string) to process on the pods list. Available filters:
    ///   - `id=<pod-id>` Matches all of pod id.
    ///   - `label=<key>` or `label=<key>:<value>` Matches pods based on the presence of a label alone or a label and a value.
    ///   - `name=<pod-name>` Matches all of pod name.
    ///   - `until=<timestamp>` List pods created before this timestamp. The `<timestamp>` can be Unix timestamps, date formatted timestamps, or Go duration strings (e.g. `10m`, `1h30m`) computed relative to the daemon machine’s time.
    ///   - `status=<pod-status>` Pod's status: `stopped`, `running`, `paused`, `exited`, `dead`, `created`, `degraded`.
    ///   - `network=<pod-network>` Name or full ID of network.
    ///   - `ctr-names=<pod-ctr-names>` Container name within the pod.
    ///   - `ctr-ids=<pod-ctr-ids>` Container ID within the pod.
    ///   - `ctr-status=<pod-ctr-status>` Container status within the pod.
    ///   - `ctr-number=<pod-ctr-number>` Number of containers in the pod.
    pub filters: Option<&'a str>,
}

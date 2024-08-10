#[derive(Default, Debug)]
pub struct VolumePruneLibpod<'a> {
    /// JSON encoded value of filters (a map[string][]string) to match volumes against before pruning.
    /// Available filters:
    ///   - `until=<timestamp>` Prune volumes created before this timestamp. The `<timestamp>` can be Unix timestamps, date formatted timestamps, or Go duration strings (e.g. `10m`, `1h30m`) computed relative to the daemon machine’s time.
    ///   - `label` (`label=<key>`, `label=<key>=<value>`, `label!=<key>`, or `label!=<key>=<value>`) Prune volumes with (or without, in case `label!=...` is used) the specified labels.
    pub filters: Option<&'a str>,
}

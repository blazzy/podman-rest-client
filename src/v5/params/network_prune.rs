#[derive(Default, Debug)]
pub struct NetworkPrune<'a> {
    /// Filters to process on the prune list, encoded as JSON (a map[string][]string).
    /// Available filters:
    ///   - `until=<timestamp>` Prune networks created before this timestamp. The <timestamp> can be Unix timestamps, date formatted timestamps, or Go duration strings (e.g. `10m`, `1h30m`) computed relative to the daemon machine’s time.
    ///   - `label` (`label=<key>`, `label=<key>=<value>`, `label!=<key>`, or `label!=<key>=<value>`) Prune networks with (or without, in case `label!=...` is used) the specified labels.
    pub filters: Option<&'a str>,
}

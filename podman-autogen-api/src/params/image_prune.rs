#[derive(Default, Debug)]
pub struct ImagePrune<'a> {
    /// filters to apply to image pruning, encoded as JSON (map[string][]string). Available filters:
    ///   - `dangling=<boolean>` When set to `true` (or `1`), prune only
    ///      unused *and* untagged images. When set to `false`
    ///      (or `0`), all unused images are pruned.
    ///   - `until=<string>` Prune images created before this timestamp. The `<timestamp>` can be Unix timestamps, date formatted timestamps, or Go duration strings (e.g. `10m`, `1h30m`) computed relative to the daemon machine’s time.
    ///   - `label` (`label=<key>`, `label=<key>=<value>`, `label!=<key>`, or `label!=<key>=<value>`) Prune images with (or without, in case `label!=...` is used) the specified labels.
    pub filters: Option<&'a str>,
}

#[derive(Default, Debug)]
pub struct VolumeListLibpod<'a> {
    /// JSON encoded value of the filters (a map[string][]string) to process on the volumes list. Available filters:
    ///   - driver=<volume-driver-name> Matches volumes based on their driver.
    ///   - label=<key> or label=<key>:<value> Matches volumes based on the presence of a label alone or a label and a value.
    ///   - name=<volume-name> Matches all of volume name.
    ///   - opt=<driver-option> Matches a storage driver options
    ///   - `until=<timestamp>` List volumes created before this timestamp. The `<timestamp>` can be Unix timestamps, date formatted timestamps, or Go duration strings (e.g. `10m`, `1h30m`) computed relative to the daemon machine’s time.
    pub filters: Option<&'a str>,
}

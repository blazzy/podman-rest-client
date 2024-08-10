#[derive(Default, Debug)]
pub struct NetworkListLibpod<'a> {
    /// JSON encoded value of the filters (a `map[string][]string`) to process on the network list. Available filters:
    ///   - `name=[name]` Matches network name (accepts regex).
    ///   - `id=[id]` Matches for full or partial ID.
    ///   - `driver=[driver]` Only bridge is supported.
    ///   - `label=[key]` or `label=[key=value]` Matches networks based on the presence of a label alone or a label and a value.
    ///   - `until=[timestamp]` Matches all networks that were created before the given timestamp.
    pub filters: Option<&'a str>,
}

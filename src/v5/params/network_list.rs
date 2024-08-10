#[derive(Default, Debug)]
pub struct NetworkList<'a> {
    /// JSON encoded value of the filters (a `map[string][]string`) to process on the network list. Currently available filters:
    ///   - `name=[name]` Matches network name (accepts regex).
    ///   - `id=[id]` Matches for full or partial ID.
    ///   - `driver=[driver]` Only bridge is supported.
    ///   - `label=[key]` or `label=[key=value]` Matches networks based on the presence of a label alone or a label and a value.
    pub filters: Option<&'a str>,
}

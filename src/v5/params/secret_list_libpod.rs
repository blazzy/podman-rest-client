#[derive(Default, Debug)]
pub struct SecretListLibpod<'a> {
    /// JSON encoded value of the filters (a `map[string][]string`) to process on the secrets list. Currently available filters:
    ///   - `name=[name]` Matches secrets name (accepts regex).
    ///   - `id=[id]` Matches for full or partial ID.
    pub filters: Option<&'a str>,
}

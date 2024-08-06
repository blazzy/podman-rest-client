#[derive(Default, Debug)]
pub struct ImageSearch<'a> {
    /// term to search
    pub term: Option<&'a str>,

    /// maximum number of results
    pub limit: Option<i64>,

    /// A JSON encoded value of the filters (a `map[string][]string`) to process on the images list. Available filters:
    /// - `is-automated=(true|false)`
    /// - `is-official=(true|false)`
    /// - `stars=<number>` Matches images that have at least 'number' stars.
    pub filters: Option<&'a str>,

    /// Require HTTPS and verify signatures when contacting registries.
    pub tls_verify: Option<bool>,

    /// list the available tags in the repository
    pub list_tags: Option<bool>,
}

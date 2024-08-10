#[derive(Default, Debug)]
pub struct SystemEvents<'a> {
    /// start streaming events from this time
    pub since: Option<&'a str>,

    /// stop streaming events later than this
    pub until: Option<&'a str>,

    /// JSON encoded map[string][]string of constraints
    pub filters: Option<&'a str>,
}

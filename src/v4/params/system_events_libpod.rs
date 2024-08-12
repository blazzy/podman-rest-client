#[derive(Default, Debug)]
pub struct SystemEventsLibpod<'a> {
    /// start streaming events from this time
    pub since: Option<&'a str>,
    /// stop streaming events later than this
    pub until: Option<&'a str>,
    /// JSON encoded map[string][]string of constraints
    pub filters: Option<&'a str>,
    /// when false, do not follow events
    pub stream: Option<bool>,
}

#[derive(Default, Debug)]
pub struct ContainersStatsAllLibpod<'a> {
    /// names or IDs of containers
    pub containers: Option<Vec<&'a str>>,

    /// Stream the output
    pub stream: Option<bool>,

    /// Time in seconds between stats reports
    pub interval: Option<i64>,
}

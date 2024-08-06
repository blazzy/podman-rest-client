#[derive(Default, Debug)]
pub struct PodStatsAllLibpod<'a> {
    /// Provide statistics for all running pods.
    pub all: Option<bool>,

    /// Names or IDs of pods.
    pub names_or_i_ds: Option<Vec<&'a str>>,
}

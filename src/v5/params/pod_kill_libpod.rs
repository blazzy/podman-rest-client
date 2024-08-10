#[derive(Default, Debug)]
pub struct PodKillLibpod<'a> {
    /// signal to be sent to pod
    pub signal: Option<&'a str>,
}

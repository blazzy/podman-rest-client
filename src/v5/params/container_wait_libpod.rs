#[derive(Default, Debug)]
pub struct ContainerWaitLibpod<'a> {
    /// Conditions to wait for. If no condition provided the 'exited' condition is assumed.
    pub condition: Option<Vec<&'a str>>,
    /// Time Interval to wait before polling for completion.
    pub interval: Option<&'a str>,
}

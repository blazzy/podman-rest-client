#[derive(Default, Debug)]
pub struct ContainerWait<'a> {
    /// wait until container is to a given condition. default is stopped. valid conditions are:
    ///   - configured
    ///   - created
    ///   - exited
    ///   - paused
    ///   - running
    ///   - stopped
    pub condition: Option<&'a str>,

    /// Time Interval to wait before polling for completion.
    pub interval: Option<&'a str>,
}

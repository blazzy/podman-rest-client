#[derive(Default, Debug)]
pub struct ContainerStopLibpod {
    /// number of seconds to wait before killing container
    pub timeout: Option<i64>,
    /// do not return error if container is already stopped
    pub ignore: Option<bool>,
}

#[derive(Default, Debug)]
pub struct ContainerRestartLibpod {
    /// number of seconds to wait before killing container
    pub t: Option<i64>,
}

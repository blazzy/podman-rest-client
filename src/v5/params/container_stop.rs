#[derive(Default, Debug)]
pub struct ContainerStop {
    /// number of seconds to wait before killing container
    pub t: Option<i64>,
}

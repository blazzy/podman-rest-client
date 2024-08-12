#[derive(Default, Debug)]
pub struct ContainerResizeLibpod {
    /// Height to set for the terminal, in characters
    pub h: Option<i64>,
    /// Width to set for the terminal, in characters
    pub w: Option<i64>,
}

#[derive(Default, Debug)]
pub struct ContainerResize {
    /// Height to set for the terminal, in characters
    pub h: Option<i64>,

    /// Width to set for the terminal, in characters
    pub w: Option<i64>,

    /// Ignore containers not running errors
    pub running: Option<bool>,
}

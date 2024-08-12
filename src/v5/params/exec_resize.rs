#[derive(Default, Debug)]
pub struct ExecResize {
    /// Height of the TTY session in characters
    pub h: Option<i64>,
    /// Width of the TTY session in characters
    pub w: Option<i64>,
    /// Ignore containers not running errors
    pub running: Option<bool>,
}

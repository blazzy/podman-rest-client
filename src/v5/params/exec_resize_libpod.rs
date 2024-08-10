#[derive(Default, Debug)]
pub struct ExecResizeLibpod {
    /// Height of the TTY session in characters
    pub h: Option<i64>,

    /// Width of the TTY session in characters
    pub w: Option<i64>,
}

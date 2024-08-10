#[derive(Default, Debug)]
pub struct NetworkDeleteLibpod {
    /// remove containers associated with network
    pub force: Option<bool>,
}

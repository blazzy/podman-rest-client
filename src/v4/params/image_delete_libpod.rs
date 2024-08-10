#[derive(Default, Debug)]
pub struct ImageDeleteLibpod {
    /// remove the image even if used by containers or has other tags
    pub force: Option<bool>,
}

#[derive(Default, Debug)]
pub struct ImageDelete {
    /// remove the image even if used by containers or has other tags
    pub force: Option<bool>,
    /// do not remove dangling parent images
    pub noprune: Option<bool>,
}

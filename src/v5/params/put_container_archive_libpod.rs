#[derive(Default, Debug)]
pub struct PutContainerArchiveLibpod<'a> {
    /// Path to a directory in the container to extract
    pub path: &'a str,
    /// pause the container while copying (defaults to true)
    pub pause: Option<bool>,
}

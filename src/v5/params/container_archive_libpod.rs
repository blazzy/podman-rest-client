#[derive(Default, Debug)]
pub struct ContainerArchiveLibpod<'a> {
    /// Path to a directory in the container to extract
    pub path: &'a str,
    /// JSON encoded map[string]string to translate paths
    pub rename: Option<&'a str>,
}

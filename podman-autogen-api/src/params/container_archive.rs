#[derive(Default, Debug)]
pub struct ContainerArchive<'a> {
    /// Path to a directory in the container to extract
    pub path: &'a str,
}

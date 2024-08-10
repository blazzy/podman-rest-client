#[derive(Default, Debug)]
pub struct ContainerRenameLibpod<'a> {
    /// New name for the container
    pub name: &'a str,
}

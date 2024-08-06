#[derive(Default, Debug)]
pub struct ContainerRename<'a> {
    /// New name for the container
    pub name: &'a str,
}

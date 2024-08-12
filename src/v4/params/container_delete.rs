#[derive(Default, Debug)]
pub struct ContainerDelete {
    /// If the container is running, kill it before removing it.
    pub force: Option<bool>,
    /// Remove the volumes associated with the container.
    pub v: Option<bool>,
    /// not supported
    pub link: Option<bool>,
}

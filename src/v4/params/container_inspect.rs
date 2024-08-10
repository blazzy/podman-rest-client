#[derive(Default, Debug)]
pub struct ContainerInspect {
    /// include the size of the container
    pub size: Option<bool>,
}

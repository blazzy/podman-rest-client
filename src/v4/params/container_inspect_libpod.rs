#[derive(Default, Debug)]
pub struct ContainerInspectLibpod {
    /// display filesystem usage
    pub size: Option<bool>,
}

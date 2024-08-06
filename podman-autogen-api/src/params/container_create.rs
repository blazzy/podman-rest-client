#[derive(Default, Debug)]
pub struct ContainerCreate<'a> {
    /// container name
    pub name: Option<&'a str>,
}

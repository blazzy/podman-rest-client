#[derive(Default, Debug)]
pub struct ContainerKill<'a> {
    /// Send kill signal to all containers
    pub all: Option<bool>,
    /// signal to be sent to container
    pub signal: Option<&'a str>,
}

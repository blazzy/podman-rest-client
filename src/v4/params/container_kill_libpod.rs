#[derive(Default, Debug)]
pub struct ContainerKillLibpod<'a> {
    /// signal to be sent to container, either by integer or SIG_ name
    pub signal: Option<&'a str>,
}

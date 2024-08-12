#[derive(Default, Debug)]
pub struct ContainerTopLibpod<'a> {
    /// when true, repeatedly stream the latest output (As of version 4.0)
    pub stream: Option<bool>,
    /// if streaming, delay in seconds between updates. Must be >1. (As of version 4.0)
    pub delay: Option<i64>,
    /// arguments to pass to ps such as aux.
    pub ps_args: Option<Vec<&'a str>>,
}

#[derive(Default, Debug)]
pub struct ContainerDeleteLibpod {
    /// additionally remove containers that depend on the container to be removed
    pub depend: Option<bool>,
    /// force stop container if running
    pub force: Option<bool>,
    /// ignore errors when the container to be removed does not existxo
    pub ignore: Option<bool>,
    /// number of seconds to wait before killing container when force removing
    pub timeout: Option<i64>,
    /// delete volumes
    pub v: Option<bool>,
}

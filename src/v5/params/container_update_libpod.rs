#[derive(Default, Debug)]
pub struct ContainerUpdateLibpod<'a> {
    /// New restart policy for the container.
    pub restart_policy: Option<&'a str>,

    /// New amount of retries for the container's restart policy. Only allowed if restartPolicy is set to on-failure
    pub restart_retries: Option<i64>,
}

#[derive(Default, Debug)]
pub struct GenerateKubeLibpod<'a> {
    /// Name or ID of the container or pod.
    pub names: Vec<&'a str>,

    /// Generate YAML for a Kubernetes service object.
    pub service: Option<bool>,

    /// Generate YAML for the given Kubernetes kind.
    pub r#type: Option<&'a str>,

    /// Set the replica number for Deployment kind.
    pub replicas: Option<i64>,

    /// don't truncate annotations to the Kubernetes maximum length of 63 characters
    pub no_trunc: Option<bool>,

    /// add podman-only reserved annotations in generated YAML file (cannot be used by Kubernetes)
    pub podman_only: Option<bool>,
}

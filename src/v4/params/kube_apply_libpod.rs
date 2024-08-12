#[derive(Default, Debug)]
pub struct KubeApplyLibpod<'a> {
    /// Path to the CA cert file for the Kubernetes cluster.
    pub ca_cert_file: Option<&'a str>,
    /// Path to the kubeconfig file for the Kubernetes cluster.
    pub kube_config: Option<&'a str>,
    /// The namespace to deploy the workload to on the Kubernetes cluster.
    pub namespace: Option<&'a str>,
    /// Create a service object for the container being deployed.
    pub service: Option<bool>,
    /// Path to the Kubernetes yaml file to deploy.
    pub file: Option<&'a str>,
}

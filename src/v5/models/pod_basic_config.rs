use serde::{Deserialize, Serialize};
#[derive(Default, Debug, Serialize, Deserialize)]
/// PodBasicConfig contains basic configuration options for pods.
pub struct PodBasicConfig {
    /// ExitPolicy determines the pod's exit and stop behaviour.
    pub exit_policy: Option<String>,
    /// Hostname is the pod's hostname. If not set, the name of the pod will
    /// be used (if a name was not provided here, the name auto-generated for
    /// the pod will be used). This will be used by the infra container and
    /// all containers in the pod as long as the UTS namespace is shared.
    /// Optional.
    pub hostname: Option<String>,
    /// InfraCommand sets the command that will be used to start the infra
    /// container.
    /// If not set, the default set in the Libpod configuration file will be
    /// used.
    /// Conflicts with NoInfra=true.
    /// Optional.
    pub infra_command: Option<Vec<String>>,
    /// InfraConmonPidFile is a custom path to store the infra container's
    /// conmon PID.
    pub infra_conmon_pid_file: Option<String>,
    /// InfraImage is the image that will be used for the infra container.
    /// If not set, the default set in the Libpod configuration file will be
    /// used.
    /// Conflicts with NoInfra=true.
    /// Optional.
    pub infra_image: Option<String>,
    /// InfraName is the name that will be used for the infra container.
    /// If not set, the default set in the Libpod configuration file will be
    /// used.
    /// Conflicts with NoInfra=true.
    /// Optional.
    pub infra_name: Option<String>,
    pub ipcns: Option<crate::v5::models::Namespace>,
    /// Labels are key-value pairs that are used to add metadata to pods.
    /// Optional.
    pub labels: Option<std::collections::HashMap<String, String>>,
    /// Name is the name of the pod.
    /// If not provided, a name will be generated when the pod is created.
    /// Optional.
    pub name: Option<String>,
    /// NoInfra tells the pod not to create an infra container. If this is
    /// done, many networking-related options will become unavailable.
    /// Conflicts with setting any options in PodNetworkConfig, and the
    /// InfraCommand and InfraImages in this struct.
    /// Optional.
    pub no_infra: Option<bool>,
    pub pidns: Option<crate::v5::models::Namespace>,
    pub pod_create_command: Option<Vec<String>>,
    /// Devices contains user specified Devices to be added to the Pod
    pub pod_devices: Option<Vec<String>>,
    /// RestartPolicy is the pod's restart policy - an action which
    /// will be taken when one or all the containers in the pod exits.
    /// If not given, the default policy will be set to Always, which
    /// restarts the containers in the pod when they exit indefinitely.
    /// Optional.
    pub restart_policy: Option<String>,
    /// RestartRetries is the number of attempts that will be made to restart
    /// the container.
    /// Only available when RestartPolicy is set to "on-failure".
    /// Optional.
    pub restart_tries: Option<u64>,
    /// PodCreateCommand is the command used to create this pod.
    /// This will be shown in the output of Inspect() on the pod, and may
    /// also be used by some tools that wish to recreate the pod
    /// (e.g. `podman generate systemd --new`).
    /// Optional.
    /// ShareParent determines if all containers in the pod will share the pod's cgroup as the cgroup parent
    pub share_parent: Option<bool>,
    /// SharedNamespaces instructs the pod to share a set of namespaces.
    /// Shared namespaces will be joined (by default) by every container
    /// which joins the pod.
    /// If not set and NoInfra is false, the pod will set a default set of
    /// namespaces to share.
    /// Conflicts with NoInfra=true.
    /// Optional.
    pub shared_namespaces: Option<Vec<String>>,
    /// Sysctl sets kernel parameters for the pod
    pub sysctl: Option<std::collections::HashMap<String, String>>,
    pub userns: Option<crate::v5::models::Namespace>,
    pub utsns: Option<crate::v5::models::Namespace>,
}

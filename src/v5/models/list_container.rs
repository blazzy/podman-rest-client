use serde::{Deserialize, Serialize};
#[derive(Default, Debug, Serialize, Deserialize)]
/// ListContainer describes a container suitable for listing
pub struct ListContainer {
    /// AutoRemove
    #[serde(rename = "AutoRemove")]
    pub auto_remove: Option<bool>,
    /// CIDFile specified at creation time.
    #[serde(rename = "CIDFile")]
    pub cid_file: Option<String>,
    /// Container command
    #[serde(rename = "Command")]
    pub command: Option<Vec<String>>,
    /// Container creation time
    #[serde(rename = "Created")]
    pub created: Option<String>,
    /// Human-readable container creation time.
    #[serde(rename = "CreatedAt")]
    pub created_at: Option<String>,
    /// If container has exited, the return code from the command
    #[serde(rename = "ExitCode")]
    pub exit_code: Option<i32>,
    /// If container has exited/stopped
    #[serde(rename = "Exited")]
    pub exited: Option<bool>,
    /// Time container exited
    #[serde(rename = "ExitedAt")]
    pub exited_at: Option<i64>,
    /// ExposedPorts contains the ports that are exposed but not forwarded,
    /// see Ports for forwarded ports.
    /// The key is the port number and the string slice contains the protocols,
    /// i.e. "tcp", "udp" and "sctp".
    #[serde(rename = "ExposedPorts")]
    pub exposed_ports: Option<serde_json::Value>,
    /// The unique identifier for the container
    #[serde(rename = "Id")]
    pub id: Option<String>,
    /// Container image
    #[serde(rename = "Image")]
    pub image: Option<String>,
    /// Container image ID
    #[serde(rename = "ImageID")]
    pub image_id: Option<String>,
    /// If this container is a Pod infra container
    #[serde(rename = "IsInfra")]
    pub is_infra: Option<bool>,
    /// Labels for container
    #[serde(rename = "Labels")]
    pub labels: Option<std::collections::HashMap<String, String>>,
    /// User volume mounts
    #[serde(rename = "Mounts")]
    pub mounts: Option<Vec<String>>,
    /// The names assigned to the container
    #[serde(rename = "Names")]
    pub names: Option<Vec<String>>,
    #[serde(rename = "Namespaces")]
    pub namespaces: Option<crate::v5::models::ListContainerNamespaces>,
    /// The network names assigned to the container
    #[serde(rename = "Networks")]
    pub networks: Option<Vec<String>>,
    /// The process id of the container
    #[serde(rename = "Pid")]
    pub pid: Option<i64>,
    /// If the container is part of Pod, the Pod ID. Requires the pod
    /// boolean to be set
    #[serde(rename = "Pod")]
    pub pod: Option<String>,
    /// If the container is part of Pod, the Pod name. Requires the pod
    /// boolean to be set
    #[serde(rename = "PodName")]
    pub pod_name: Option<String>,
    /// Port mappings
    #[serde(rename = "Ports")]
    pub ports: Option<Vec<crate::v5::models::PortMapping>>,
    /// Restarts is how many times the container was restarted by its
    /// restart policy. This is NOT incremented by normal container restarts
    /// (only by restart policy).
    #[serde(rename = "Restarts")]
    pub restarts: Option<u64>,
    #[serde(rename = "Size")]
    pub size: Option<crate::v5::models::ContainerSize>,
    /// Time when container started
    #[serde(rename = "StartedAt")]
    pub started_at: Option<i64>,
    /// State of container
    #[serde(rename = "State")]
    pub state: Option<String>,
    /// Status is a human-readable approximation of a duration for json output
    #[serde(rename = "Status")]
    pub status: Option<String>,
}

use serde::{Deserialize, Serialize};
#[derive(Default, Debug, Serialize, Deserialize)]
/// InspectPodData contains detailed information on a pod's configuration and
/// state. It is used as the output of Inspect on pods.
pub struct InspectPodData {
    /// CgroupParent is the parent of the pod's Cgroup.
    #[serde(rename = "CgroupParent")]
    pub cgroup_parent: Option<String>,
    /// CgroupPath is the path to the pod's Cgroup.
    #[serde(rename = "CgroupPath")]
    pub cgroup_path: Option<String>,
    /// Containers gives a brief summary of all containers in the pod and
    /// their current status.
    #[serde(rename = "Containers")]
    pub containers: Option<Vec<super::super::models::InspectPodContainerInfo>>,
    /// CreateCgroup is whether this pod will create its own Cgroup to group
    /// containers under.
    #[serde(rename = "CreateCgroup")]
    pub create_cgroup: Option<bool>,
    /// CreateCommand is the full command plus arguments of the process the
    /// container has been created with.
    #[serde(rename = "CreateCommand")]
    pub create_command: Option<Vec<String>>,
    /// CreateInfra is whether this pod will create an infra container to
    /// share namespaces.
    #[serde(rename = "CreateInfra")]
    pub create_infra: Option<bool>,
    /// Created is the time when the pod was created.
    #[serde(rename = "Created")]
    pub created: Option<String>,
    /// ExitPolicy of the pod.
    #[serde(rename = "ExitPolicy")]
    pub exit_policy: Option<String>,
    /// Hostname is the hostname that the pod will set.
    #[serde(rename = "Hostname")]
    pub hostname: Option<String>,
    /// ID is the ID of the pod.
    #[serde(rename = "Id")]
    pub id: Option<String>,
    #[serde(rename = "InfraConfig")]
    pub infra_config: Option<super::super::models::InspectPodInfraConfig>,
    /// InfraContainerID is the ID of the pod's infra container, if one is
    /// present.
    #[serde(rename = "InfraContainerID")]
    pub infra_container_id: Option<String>,
    /// Labels is a set of key-value labels that have been applied to the
    /// pod.
    #[serde(rename = "Labels")]
    pub labels: Option<std::collections::HashMap<String, String>>,
    /// Number of the pod's Libpod lock.
    #[serde(rename = "LockNumber")]
    pub lock_number: Option<u32>,
    /// Name is the name of the pod.
    #[serde(rename = "Name")]
    pub name: Option<String>,
    /// Namespace is the Libpod namespace the pod is placed in.
    #[serde(rename = "Namespace")]
    pub namespace: Option<String>,
    /// NumContainers is the number of containers in the pod, including the
    /// infra container.
    #[serde(rename = "NumContainers")]
    pub num_containers: Option<u64>,
    /// RestartPolicy of the pod.
    #[serde(rename = "RestartPolicy")]
    pub restart_policy: Option<String>,
    /// SharedNamespaces contains a list of namespaces that will be shared by
    /// containers within the pod. Can only be set if CreateInfra is true.
    #[serde(rename = "SharedNamespaces")]
    pub shared_namespaces: Option<Vec<String>>,
    /// State represents the current state of the pod.
    #[serde(rename = "State")]
    pub state: Option<String>,
    /// BlkioWeight contains the blkio weight limit for the pod
    pub blkio_weight: Option<u64>,
    /// BlkioWeightDevice contains the blkio weight device limits for the pod
    pub blkio_weight_device: Option<Vec<super::super::models::InspectBlkioWeightDevice>>,
    /// CPUPeriod contains the CPU period of the pod
    pub cpu_period: Option<u64>,
    /// CPUQuota contains the CPU quota of the pod
    pub cpu_quota: Option<i64>,
    /// CPUShares contains the cpu shares for the pod
    pub cpu_shares: Option<u64>,
    /// CPUSetCPUs contains linux specific CPU data for the pod
    pub cpuset_cpus: Option<String>,
    /// CPUSetMems contains linux specific CPU data for the pod
    pub cpuset_mems: Option<String>,
    /// BlkioDeviceReadBps contains the Read/Access limit for the pod's devices
    pub device_read_bps: Option<Vec<super::super::models::InspectBlkioThrottleDevice>>,
    /// BlkioDeviceReadBps contains the Read/Access limit for the pod's devices
    pub device_write_bps: Option<Vec<super::super::models::InspectBlkioThrottleDevice>>,
    /// Devices contains the specified host devices
    pub devices: Option<Vec<super::super::models::InspectDevice>>,
    /// MemoryLimit contains the specified cgroup memory limit for the pod
    pub memory_limit: Option<u64>,
    /// MemorySwap contains the specified memory swap limit for the pod
    pub memory_swap: Option<u64>,
    /// Mounts contains volume related information for the pod
    pub mounts: Option<Vec<super::super::models::InspectMount>>,
    /// SecurityOpt contains the specified security labels and related SELinux information
    pub security_opt: Option<Vec<String>>,
    /// VolumesFrom contains the containers that the pod inherits mounts from
    pub volumes_from: Option<Vec<String>>,
}

/*
 * supports a RESTful API for the Libpod library
 *
 * This documentation describes the Podman v2.x+ RESTful API. It consists of a Docker-compatible API and a Libpod API providing support for Podman’s unique features such as pods.  To start the service and keep it running for 5,000 seconds (-t 0 runs forever):  podman system service -t 5000 &  You can then use cURL on the socket using requests documented below.  NOTE: if you install the package podman-docker, it will create a symbolic link for /run/docker.sock to /run/podman/podman.sock  NOTE: Some fields in the API response JSON are encoded as omitempty, which means that if said field has a zero value, they will not be encoded in the API response. This is a feature to help reduce the size of the JSON responses returned via the API.  NOTE: Due to the limitations of [go-swagger](https://github.com/go-swagger/go-swagger), some field values that have a complex type show up as null in the docs as well as in the API responses. This is because the zero value for the field type is null. The field description in the docs will state what type the field is expected to be for such cases.  See podman-system-service(1) for more information.  Quick Examples:  'podman info'  curl --unix-socket /run/podman/podman.sock http://d/v5.0.0/libpod/info  'podman pull quay.io/containers/podman'  curl -XPOST --unix-socket /run/podman/podman.sock -v 'http://d/v5.0.0/images/create?fromImage=quay.io%2Fcontainers%2Fpodman'  'podman list images'  curl --unix-socket /run/podman/podman.sock -v 'http://d/v5.0.0/libpod/images/json' | jq
 *
 * The version of the OpenAPI document: 5.0.0
 * Contact: podman@lists.podman.io
 * Generated by: https://openapi-generator.tech
 */

use crate::models;
use serde::{Deserialize, Serialize};

/// InspectPodData : InspectPodData contains detailed information on a pod's configuration and state. It is used as the output of Inspect on pods.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct InspectPodData {
    /// CgroupParent is the parent of the pod's Cgroup.
    #[serde(rename = "CgroupParent", skip_serializing_if = "Option::is_none")]
    pub cgroup_parent: Option<String>,
    /// CgroupPath is the path to the pod's Cgroup.
    #[serde(rename = "CgroupPath", skip_serializing_if = "Option::is_none")]
    pub cgroup_path: Option<String>,
    /// Containers gives a brief summary of all containers in the pod and their current status.
    #[serde(rename = "Containers", skip_serializing_if = "Option::is_none")]
    pub containers: Option<Vec<models::InspectPodContainerInfo>>,
    /// CreateCgroup is whether this pod will create its own Cgroup to group containers under.
    #[serde(rename = "CreateCgroup", skip_serializing_if = "Option::is_none")]
    pub create_cgroup: Option<bool>,
    /// CreateCommand is the full command plus arguments of the process the container has been created with.
    #[serde(rename = "CreateCommand", skip_serializing_if = "Option::is_none")]
    pub create_command: Option<Vec<String>>,
    /// CreateInfra is whether this pod will create an infra container to share namespaces.
    #[serde(rename = "CreateInfra", skip_serializing_if = "Option::is_none")]
    pub create_infra: Option<bool>,
    /// Created is the time when the pod was created.
    #[serde(rename = "Created", skip_serializing_if = "Option::is_none")]
    pub created: Option<String>,
    /// ExitPolicy of the pod.
    #[serde(rename = "ExitPolicy", skip_serializing_if = "Option::is_none")]
    pub exit_policy: Option<String>,
    /// Hostname is the hostname that the pod will set.
    #[serde(rename = "Hostname", skip_serializing_if = "Option::is_none")]
    pub hostname: Option<String>,
    /// ID is the ID of the pod.
    #[serde(rename = "Id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "InfraConfig", skip_serializing_if = "Option::is_none")]
    pub infra_config: Option<Box<models::InspectPodInfraConfig>>,
    /// InfraContainerID is the ID of the pod's infra container, if one is present.
    #[serde(rename = "InfraContainerID", skip_serializing_if = "Option::is_none")]
    pub infra_container_id: Option<String>,
    /// Labels is a set of key-value labels that have been applied to the pod.
    #[serde(rename = "Labels", skip_serializing_if = "Option::is_none")]
    pub labels: Option<std::collections::HashMap<String, String>>,
    /// Number of the pod's Libpod lock.
    #[serde(rename = "LockNumber", skip_serializing_if = "Option::is_none")]
    pub lock_number: Option<i32>,
    /// Name is the name of the pod.
    #[serde(rename = "Name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Namespace is the Libpod namespace the pod is placed in.
    #[serde(rename = "Namespace", skip_serializing_if = "Option::is_none")]
    pub namespace: Option<String>,
    /// NumContainers is the number of containers in the pod, including the infra container.
    #[serde(rename = "NumContainers", skip_serializing_if = "Option::is_none")]
    pub num_containers: Option<i32>,
    /// RestartPolicy of the pod.
    #[serde(rename = "RestartPolicy", skip_serializing_if = "Option::is_none")]
    pub restart_policy: Option<String>,
    /// SharedNamespaces contains a list of namespaces that will be shared by containers within the pod. Can only be set if CreateInfra is true.
    #[serde(rename = "SharedNamespaces", skip_serializing_if = "Option::is_none")]
    pub shared_namespaces: Option<Vec<String>>,
    /// State represents the current state of the pod.
    #[serde(rename = "State", skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    /// BlkioWeight contains the blkio weight limit for the pod
    #[serde(rename = "blkio_weight", skip_serializing_if = "Option::is_none")]
    pub blkio_weight: Option<i32>,
    /// BlkioWeightDevice contains the blkio weight device limits for the pod
    #[serde(
        rename = "blkio_weight_device",
        skip_serializing_if = "Option::is_none"
    )]
    pub blkio_weight_device: Option<Vec<models::InspectBlkioWeightDevice>>,
    /// CPUPeriod contains the CPU period of the pod
    #[serde(rename = "cpu_period", skip_serializing_if = "Option::is_none")]
    pub cpu_period: Option<i32>,
    /// CPUQuota contains the CPU quota of the pod
    #[serde(rename = "cpu_quota", skip_serializing_if = "Option::is_none")]
    pub cpu_quota: Option<i64>,
    /// CPUShares contains the cpu shares for the pod
    #[serde(rename = "cpu_shares", skip_serializing_if = "Option::is_none")]
    pub cpu_shares: Option<i32>,
    /// CPUSetCPUs contains linux specific CPU data for the pod
    #[serde(rename = "cpuset_cpus", skip_serializing_if = "Option::is_none")]
    pub cpuset_cpus: Option<String>,
    /// CPUSetMems contains linux specific CPU data for the pod
    #[serde(rename = "cpuset_mems", skip_serializing_if = "Option::is_none")]
    pub cpuset_mems: Option<String>,
    /// BlkioDeviceReadBps contains the Read/Access limit for the pod's devices
    #[serde(rename = "device_read_bps", skip_serializing_if = "Option::is_none")]
    pub device_read_bps: Option<Vec<models::InspectBlkioThrottleDevice>>,
    /// BlkioDeviceReadBps contains the Read/Access limit for the pod's devices
    #[serde(rename = "device_write_bps", skip_serializing_if = "Option::is_none")]
    pub device_write_bps: Option<Vec<models::InspectBlkioThrottleDevice>>,
    /// Devices contains the specified host devices
    #[serde(rename = "devices", skip_serializing_if = "Option::is_none")]
    pub devices: Option<Vec<models::InspectDevice>>,
    /// MemoryLimit contains the specified cgroup memory limit for the pod
    #[serde(rename = "memory_limit", skip_serializing_if = "Option::is_none")]
    pub memory_limit: Option<i32>,
    /// MemorySwap contains the specified memory swap limit for the pod
    #[serde(rename = "memory_swap", skip_serializing_if = "Option::is_none")]
    pub memory_swap: Option<i32>,
    /// Mounts contains volume related information for the pod
    #[serde(rename = "mounts", skip_serializing_if = "Option::is_none")]
    pub mounts: Option<Vec<models::InspectMount>>,
    /// SecurityOpt contains the specified security labels and related SELinux information
    #[serde(rename = "security_opt", skip_serializing_if = "Option::is_none")]
    pub security_opt: Option<Vec<String>>,
    /// VolumesFrom contains the containers that the pod inherits mounts from
    #[serde(rename = "volumes_from", skip_serializing_if = "Option::is_none")]
    pub volumes_from: Option<Vec<String>>,
}

impl InspectPodData {
    /// InspectPodData contains detailed information on a pod's configuration and state. It is used as the output of Inspect on pods.
    pub fn new() -> InspectPodData {
        InspectPodData {
            cgroup_parent: None,
            cgroup_path: None,
            containers: None,
            create_cgroup: None,
            create_command: None,
            create_infra: None,
            created: None,
            exit_policy: None,
            hostname: None,
            id: None,
            infra_config: None,
            infra_container_id: None,
            labels: None,
            lock_number: None,
            name: None,
            namespace: None,
            num_containers: None,
            restart_policy: None,
            shared_namespaces: None,
            state: None,
            blkio_weight: None,
            blkio_weight_device: None,
            cpu_period: None,
            cpu_quota: None,
            cpu_shares: None,
            cpuset_cpus: None,
            cpuset_mems: None,
            device_read_bps: None,
            device_write_bps: None,
            devices: None,
            memory_limit: None,
            memory_swap: None,
            mounts: None,
            security_opt: None,
            volumes_from: None,
        }
    }
}

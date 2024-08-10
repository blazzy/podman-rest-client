use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Serialize, Deserialize)]
/// PodCreateOptions provides all possible options for creating a pod and its infra container.
/// The JSON tags below are made to match the respective field in ContainerCreateOptions for the purpose of mapping.
pub struct PodCreateOptions {
    pub cgroup_parent: Option<String>,

    pub container_command: Option<String>,

    pub container_conmon_pidfile: Option<String>,

    pub container_name: Option<String>,

    pub cpus: Option<f64>,

    pub cpuset_cpus: Option<String>,

    pub create_command: Option<Vec<String>>,

    pub device_read_bps: Option<Vec<String>>,

    pub devices: Option<Vec<String>>,

    pub exit_policy: Option<String>,

    pub hostname: Option<String>,

    pub infra: Option<bool>,

    pub infra_image: Option<String>,

    pub ipc: Option<String>,

    pub labels: Option<std::collections::HashMap<String, String>>,

    pub name: Option<String>,

    pub net: Option<super::super::models::NetOptions>,

    pub pid: Option<String>,

    pub restart: Option<String>,

    pub security_opt: Option<Vec<String>>,

    pub share: Option<Vec<String>>,

    pub share_parent: Option<bool>,

    pub sysctl: Option<Vec<String>>,

    pub uts: Option<String>,

    pub volume: Option<Vec<String>>,

    pub volumes_from: Option<Vec<String>>,
}

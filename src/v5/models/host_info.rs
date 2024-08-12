use serde::{Deserialize, Serialize};
#[derive(Default, Debug, Serialize, Deserialize)]
/// HostInfo describes the libpod host
pub struct HostInfo {
    pub arch: Option<String>,
    #[serde(rename = "buildahVersion")]
    pub buildah_version: Option<String>,
    #[serde(rename = "cgroupControllers")]
    pub cgroup_controllers: Option<Vec<String>>,
    #[serde(rename = "cgroupManager")]
    pub cgroup_manager: Option<String>,
    #[serde(rename = "cgroupVersion")]
    pub cgroup_version: Option<String>,
    pub conmon: Option<super::super::models::ConmonInfo>,
    #[serde(rename = "cpuUtilization")]
    pub cpu_utilization: Option<super::super::models::CpuUsage>,
    pub cpus: Option<i64>,
    #[serde(rename = "databaseBackend")]
    pub database_backend: Option<String>,
    pub distribution: Option<super::super::models::DistributionInfo>,
    #[serde(rename = "eventLogger")]
    pub event_logger: Option<String>,
    #[serde(rename = "freeLocks")]
    pub free_locks: Option<u32>,
    pub hostname: Option<String>,
    #[serde(rename = "idMappings")]
    pub id_mappings: Option<super::super::models::IdMappings>,
    pub kernel: Option<String>,
    pub linkmode: Option<String>,
    #[serde(rename = "logDriver")]
    pub log_driver: Option<String>,
    #[serde(rename = "memFree")]
    pub mem_free: Option<i64>,
    #[serde(rename = "memTotal")]
    pub mem_total: Option<i64>,
    #[serde(rename = "networkBackend")]
    pub network_backend: Option<String>,
    #[serde(rename = "networkBackendInfo")]
    pub network_backend_info: Option<super::super::models::NetworkInfo>,
    #[serde(rename = "ociRuntime")]
    pub oci_runtime: Option<super::super::models::OciRuntimeInfo>,
    pub os: Option<String>,
    pub pasta: Option<super::super::models::PastaInfo>,
    #[serde(rename = "remoteSocket")]
    pub remote_socket: Option<super::super::models::RemoteSocket>,
    /// RootlessNetworkCmd returns the default rootless network command (slirp4netns or pasta)
    #[serde(rename = "rootlessNetworkCmd")]
    pub rootless_network_cmd: Option<String>,
    #[serde(rename = "runtimeInfo")]
    pub runtime_info: Option<std::collections::HashMap<String, ()>>,
    pub security: Option<super::super::models::SecurityInfo>,
    /// ServiceIsRemote is true when the podman/libpod service is remote to the client
    #[serde(rename = "serviceIsRemote")]
    pub service_is_remote: Option<bool>,
    #[serde(rename = "slirp4netns")]
    pub slirp_4_netns: Option<super::super::models::SlirpInfo>,
    #[serde(rename = "swapFree")]
    pub swap_free: Option<i64>,
    #[serde(rename = "swapTotal")]
    pub swap_total: Option<i64>,
    pub uptime: Option<String>,
    pub variant: Option<String>,
}

use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Serialize, Deserialize)]
/// LinuxCPU for Linux cgroup 'cpu' resource management
pub struct LinuxCpu {
    /// CPU hardcap burst limit (in usecs). Allowed accumulated cpu time additionally for burst in a
    /// given period.
    pub burst: Option<u64>,

    /// CPUs to use within the cpuset. Default is to use any CPU available.
    pub cpus: Option<String>,

    /// cgroups are configured with minimum weight, 0: default behavior, 1: SCHED_IDLE.
    pub idle: Option<i64>,

    /// List of memory nodes in the cpuset. Default is to use any available memory node.
    pub mems: Option<String>,

    /// CPU period to be used for hardcapping (in usecs).
    pub period: Option<u64>,

    /// CPU hardcap limit (in usecs). Allowed cpu time in a given period.
    pub quota: Option<i64>,

    /// CPU period to be used for realtime scheduling (in usecs).
    #[serde(rename = "realtimePeriod")]
    pub realtime_period: Option<u64>,

    /// How much time realtime scheduling may use (in usecs).
    #[serde(rename = "realtimeRuntime")]
    pub realtime_runtime: Option<i64>,

    /// CPU shares (relative weight (ratio) vs. other cgroups with cpu shares).
    pub shares: Option<u64>,
}

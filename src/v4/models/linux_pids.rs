use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Serialize, Deserialize)]
/// LinuxPids for Linux cgroup 'pids' resource management (Linux 4.3)
pub struct LinuxPids {
    /// Maximum number of PIDs. Default is "no limit".
    pub limit: Option<i64>,
}

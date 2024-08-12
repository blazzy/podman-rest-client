use serde::{Deserialize, Serialize};
#[derive(Default, Debug, Serialize, Deserialize)]
/// LinuxMemory for Linux cgroup 'memory' resource management
pub struct LinuxMemory {
    /// CheckBeforeUpdate enables checking if a new memory limit is lower
    /// than the current usage during update, and if so, rejecting the new
    /// limit.
    #[serde(rename = "checkBeforeUpdate")]
    pub check_before_update: Option<bool>,
    /// DisableOOMKiller disables the OOM killer for out of memory conditions
    #[serde(rename = "disableOOMKiller")]
    pub disable_oom_killer: Option<bool>,
    /// Kernel memory limit (in bytes).
    ///
    /// Deprecated: kernel-memory limits are not supported in cgroups v2, and
    /// were obsoleted in [kernel v5.4]. This field should no longer be used,
    /// as it may be ignored by runtimes.
    ///
    /// [kernel v5.4]: https://github.com/torvalds/linux/commit/0158115f702b0ba208ab0
    pub kernel: Option<i64>,
    /// Kernel memory limit for tcp (in bytes)
    #[serde(rename = "kernelTCP")]
    pub kernel_tcp: Option<i64>,
    /// Memory limit (in bytes).
    pub limit: Option<i64>,
    /// Memory reservation or soft_limit (in bytes).
    pub reservation: Option<i64>,
    /// Total memory limit (memory + swap).
    pub swap: Option<i64>,
    /// How aggressive the kernel will swap memory pages.
    pub swappiness: Option<u64>,
    /// Enables hierarchical memory accounting
    #[serde(rename = "useHierarchy")]
    pub use_hierarchy: Option<bool>,
}

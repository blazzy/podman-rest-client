use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Serialize, Deserialize)]
/// LinuxHugepageLimit structure corresponds to limiting kernel hugepages.
/// Default to reservation limits if supported. Otherwise fallback to page fault limits.
pub struct LinuxHugepageLimit {
    /// Limit is the limit of "hugepagesize" hugetlb reservations (if supported) or usage.
    pub limit: Option<u64>,

    /// Pagesize is the hugepage size.
    /// Format: "<size><unit-prefix>B' (e.g. 64KB, 2MB, 1GB, etc.).
    #[serde(rename = "pageSize")]
    pub page_size: Option<String>,
}

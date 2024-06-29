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

/// LinuxMemory : LinuxMemory for Linux cgroup 'memory' resource management
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct LinuxMemory {
    /// CheckBeforeUpdate enables checking if a new memory limit is lower than the current usage during update, and if so, rejecting the new limit.
    #[serde(rename = "checkBeforeUpdate", skip_serializing_if = "Option::is_none")]
    pub check_before_update: Option<bool>,
    /// DisableOOMKiller disables the OOM killer for out of memory conditions
    #[serde(rename = "disableOOMKiller", skip_serializing_if = "Option::is_none")]
    pub disable_oom_killer: Option<bool>,
    /// Kernel memory limit (in bytes).  Deprecated: kernel-memory limits are not supported in cgroups v2, and were obsoleted in [kernel v5.4]. This field should no longer be used, as it may be ignored by runtimes.  [kernel v5.4]: https://github.com/torvalds/linux/commit/0158115f702b0ba208ab0
    #[serde(rename = "kernel", skip_serializing_if = "Option::is_none")]
    pub kernel: Option<i64>,
    /// Kernel memory limit for tcp (in bytes)
    #[serde(rename = "kernelTCP", skip_serializing_if = "Option::is_none")]
    pub kernel_tcp: Option<i64>,
    /// Memory limit (in bytes).
    #[serde(rename = "limit", skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    /// Memory reservation or soft_limit (in bytes).
    #[serde(rename = "reservation", skip_serializing_if = "Option::is_none")]
    pub reservation: Option<i64>,
    /// Total memory limit (memory + swap).
    #[serde(rename = "swap", skip_serializing_if = "Option::is_none")]
    pub swap: Option<i64>,
    /// How aggressive the kernel will swap memory pages.
    #[serde(rename = "swappiness", skip_serializing_if = "Option::is_none")]
    pub swappiness: Option<i32>,
    /// Enables hierarchical memory accounting
    #[serde(rename = "useHierarchy", skip_serializing_if = "Option::is_none")]
    pub use_hierarchy: Option<bool>,
}

impl LinuxMemory {
    /// LinuxMemory for Linux cgroup 'memory' resource management
    pub fn new() -> LinuxMemory {
        LinuxMemory {
            check_before_update: None,
            disable_oom_killer: None,
            kernel: None,
            kernel_tcp: None,
            limit: None,
            reservation: None,
            swap: None,
            swappiness: None,
            use_hierarchy: None,
        }
    }
}

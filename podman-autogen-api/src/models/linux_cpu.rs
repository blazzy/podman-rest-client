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

/// LinuxCpu : LinuxCPU for Linux cgroup 'cpu' resource management
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct LinuxCpu {
    /// CPU hardcap burst limit (in usecs). Allowed accumulated cpu time additionally for burst in a given period.
    #[serde(rename = "burst", skip_serializing_if = "Option::is_none")]
    pub burst: Option<i32>,
    /// CPUs to use within the cpuset. Default is to use any CPU available.
    #[serde(rename = "cpus", skip_serializing_if = "Option::is_none")]
    pub cpus: Option<String>,
    /// cgroups are configured with minimum weight, 0: default behavior, 1: SCHED_IDLE.
    #[serde(rename = "idle", skip_serializing_if = "Option::is_none")]
    pub idle: Option<i64>,
    /// List of memory nodes in the cpuset. Default is to use any available memory node.
    #[serde(rename = "mems", skip_serializing_if = "Option::is_none")]
    pub mems: Option<String>,
    /// CPU period to be used for hardcapping (in usecs).
    #[serde(rename = "period", skip_serializing_if = "Option::is_none")]
    pub period: Option<i32>,
    /// CPU hardcap limit (in usecs). Allowed cpu time in a given period.
    #[serde(rename = "quota", skip_serializing_if = "Option::is_none")]
    pub quota: Option<i64>,
    /// CPU period to be used for realtime scheduling (in usecs).
    #[serde(rename = "realtimePeriod", skip_serializing_if = "Option::is_none")]
    pub realtime_period: Option<i32>,
    /// How much time realtime scheduling may use (in usecs).
    #[serde(rename = "realtimeRuntime", skip_serializing_if = "Option::is_none")]
    pub realtime_runtime: Option<i64>,
    /// CPU shares (relative weight (ratio) vs. other cgroups with cpu shares).
    #[serde(rename = "shares", skip_serializing_if = "Option::is_none")]
    pub shares: Option<i32>,
}

impl LinuxCpu {
    /// LinuxCPU for Linux cgroup 'cpu' resource management
    pub fn new() -> LinuxCpu {
        LinuxCpu {
            burst: None,
            cpus: None,
            idle: None,
            mems: None,
            period: None,
            quota: None,
            realtime_period: None,
            realtime_runtime: None,
            shares: None,
        }
    }
}
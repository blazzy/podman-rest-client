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

/// ContainerState : ContainerState stores container's running state it's part of ContainerJSONBase and will return by \"inspect\" command
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct ContainerState {
    #[serde(rename = "Dead", skip_serializing_if = "Option::is_none")]
    pub dead: Option<bool>,
    #[serde(rename = "Error", skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
    #[serde(rename = "ExitCode", skip_serializing_if = "Option::is_none")]
    pub exit_code: Option<i64>,
    #[serde(rename = "FinishedAt", skip_serializing_if = "Option::is_none")]
    pub finished_at: Option<String>,
    #[serde(rename = "Health", skip_serializing_if = "Option::is_none")]
    pub health: Option<Box<models::Health>>,
    #[serde(rename = "OOMKilled", skip_serializing_if = "Option::is_none")]
    pub oom_killed: Option<bool>,
    #[serde(rename = "Paused", skip_serializing_if = "Option::is_none")]
    pub paused: Option<bool>,
    #[serde(rename = "Pid", skip_serializing_if = "Option::is_none")]
    pub pid: Option<i64>,
    #[serde(rename = "Restarting", skip_serializing_if = "Option::is_none")]
    pub restarting: Option<bool>,
    #[serde(rename = "Running", skip_serializing_if = "Option::is_none")]
    pub running: Option<bool>,
    #[serde(rename = "StartedAt", skip_serializing_if = "Option::is_none")]
    pub started_at: Option<String>,
    #[serde(rename = "Status", skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

impl ContainerState {
    /// ContainerState stores container's running state it's part of ContainerJSONBase and will return by \"inspect\" command
    pub fn new() -> ContainerState {
        ContainerState {
            dead: None,
            error: None,
            exit_code: None,
            finished_at: None,
            health: None,
            oom_killed: None,
            paused: None,
            pid: None,
            restarting: None,
            running: None,
            started_at: None,
            status: None,
        }
    }
}

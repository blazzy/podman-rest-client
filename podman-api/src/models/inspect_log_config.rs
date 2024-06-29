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

/// InspectLogConfig : InspectLogConfig holds information about a container's configured log driver
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct InspectLogConfig {
    #[serde(rename = "Config", skip_serializing_if = "Option::is_none")]
    pub config: Option<std::collections::HashMap<String, String>>,
    /// Path specifies a path to the log file
    #[serde(rename = "Path", skip_serializing_if = "Option::is_none")]
    pub path: Option<String>,
    /// Size specifies a maximum size of the container log
    #[serde(rename = "Size", skip_serializing_if = "Option::is_none")]
    pub size: Option<String>,
    /// Tag specifies a custom log tag for the container
    #[serde(rename = "Tag", skip_serializing_if = "Option::is_none")]
    pub tag: Option<String>,
    #[serde(rename = "Type", skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
}

impl InspectLogConfig {
    /// InspectLogConfig holds information about a container's configured log driver
    pub fn new() -> InspectLogConfig {
        InspectLogConfig {
            config: None,
            path: None,
            size: None,
            tag: None,
            r#type: None,
        }
    }
}

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

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct StartupHealthCheck {
    /// A Duration represents the elapsed time between two instants as an int64 nanosecond count. The representation limits the largest representable duration to approximately 290 years.
    #[serde(rename = "Interval", skip_serializing_if = "Option::is_none")]
    pub interval: Option<i64>,
    /// Retries is the number of consecutive failures needed to consider a container as unhealthy. Zero means inherit.
    #[serde(rename = "Retries", skip_serializing_if = "Option::is_none")]
    pub retries: Option<i64>,
    /// A Duration represents the elapsed time between two instants as an int64 nanosecond count. The representation limits the largest representable duration to approximately 290 years.
    #[serde(rename = "StartInterval", skip_serializing_if = "Option::is_none")]
    pub start_interval: Option<i64>,
    /// A Duration represents the elapsed time between two instants as an int64 nanosecond count. The representation limits the largest representable duration to approximately 290 years.
    #[serde(rename = "StartPeriod", skip_serializing_if = "Option::is_none")]
    pub start_period: Option<i64>,
    /// Successes are the number of successes required to mark the startup HC as passed. If set to 0, a single success will mark the HC as passed.
    #[serde(rename = "Successes", skip_serializing_if = "Option::is_none")]
    pub successes: Option<i64>,
    /// Test is the test to perform to check that the container is healthy. An empty slice means to inherit the default. The options are: {} : inherit healthcheck {\"NONE\"} : disable healthcheck {\"CMD\", args...} : exec arguments directly {\"CMD-SHELL\", command} : run command with system's default shell
    #[serde(rename = "Test", skip_serializing_if = "Option::is_none")]
    pub test: Option<Vec<String>>,
    /// A Duration represents the elapsed time between two instants as an int64 nanosecond count. The representation limits the largest representable duration to approximately 290 years.
    #[serde(rename = "Timeout", skip_serializing_if = "Option::is_none")]
    pub timeout: Option<i64>,
}

impl StartupHealthCheck {
    pub fn new() -> StartupHealthCheck {
        StartupHealthCheck {
            interval: None,
            retries: None,
            start_interval: None,
            start_period: None,
            successes: None,
            test: None,
            timeout: None,
        }
    }
}

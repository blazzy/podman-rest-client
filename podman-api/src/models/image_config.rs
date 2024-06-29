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
pub struct ImageConfig {
    /// ArgsEscaped  Deprecated: This field is present only for legacy compatibility with Docker and should not be used by new image builders.  It is used by Docker for Windows images to indicate that the `Entrypoint` or `Cmd` or both, contains only a single element array, that is a pre-escaped, and combined into a single string `CommandLine`. If `true` the value in `Entrypoint` or `Cmd` should be used as-is to avoid double escaping. https://github.com/opencontainers/image-spec/pull/892
    #[serde(rename = "ArgsEscaped", skip_serializing_if = "Option::is_none")]
    pub args_escaped: Option<bool>,
    /// Cmd defines the default arguments to the entrypoint of the container.
    #[serde(rename = "Cmd", skip_serializing_if = "Option::is_none")]
    pub cmd: Option<Vec<String>>,
    /// Entrypoint defines a list of arguments to use as the command to execute when the container starts.
    #[serde(rename = "Entrypoint", skip_serializing_if = "Option::is_none")]
    pub entrypoint: Option<Vec<String>>,
    /// Env is a list of environment variables to be used in a container.
    #[serde(rename = "Env", skip_serializing_if = "Option::is_none")]
    pub env: Option<Vec<String>>,
    /// ExposedPorts a set of ports to expose from a container running this image.
    #[serde(rename = "ExposedPorts", skip_serializing_if = "Option::is_none")]
    pub exposed_ports: Option<std::collections::HashMap<String, serde_json::Value>>,
    /// Labels contains arbitrary metadata for the container.
    #[serde(rename = "Labels", skip_serializing_if = "Option::is_none")]
    pub labels: Option<std::collections::HashMap<String, String>>,
    /// StopSignal contains the system call signal that will be sent to the container to exit.
    #[serde(rename = "StopSignal", skip_serializing_if = "Option::is_none")]
    pub stop_signal: Option<String>,
    /// User defines the username or UID which the process in the container should run as.
    #[serde(rename = "User", skip_serializing_if = "Option::is_none")]
    pub user: Option<String>,
    /// Volumes is a set of directories describing where the process is likely write data specific to a container instance.
    #[serde(rename = "Volumes", skip_serializing_if = "Option::is_none")]
    pub volumes: Option<std::collections::HashMap<String, serde_json::Value>>,
    /// WorkingDir sets the current working directory of the entrypoint process in the container.
    #[serde(rename = "WorkingDir", skip_serializing_if = "Option::is_none")]
    pub working_dir: Option<String>,
}

impl ImageConfig {
    pub fn new() -> ImageConfig {
        ImageConfig {
            args_escaped: None,
            cmd: None,
            entrypoint: None,
            env: None,
            exposed_ports: None,
            labels: None,
            stop_signal: None,
            user: None,
            volumes: None,
            working_dir: None,
        }
    }
}

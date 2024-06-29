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
pub struct BindOptions {
    #[serde(rename = "CreateMountpoint", skip_serializing_if = "Option::is_none")]
    pub create_mountpoint: Option<bool>,
    #[serde(rename = "NonRecursive", skip_serializing_if = "Option::is_none")]
    pub non_recursive: Option<bool>,
    #[serde(rename = "Propagation", skip_serializing_if = "Option::is_none")]
    pub propagation: Option<String>,
    /// ReadOnlyForceRecursive raises an error if the mount cannot be made recursively read-only.
    #[serde(
        rename = "ReadOnlyForceRecursive",
        skip_serializing_if = "Option::is_none"
    )]
    pub read_only_force_recursive: Option<bool>,
    /// ReadOnlyNonRecursive makes the mount non-recursively read-only, but still leaves the mount recursive (unless NonRecursive is set to true in conjunction).
    #[serde(
        rename = "ReadOnlyNonRecursive",
        skip_serializing_if = "Option::is_none"
    )]
    pub read_only_non_recursive: Option<bool>,
}

impl BindOptions {
    pub fn new() -> BindOptions {
        BindOptions {
            create_mountpoint: None,
            non_recursive: None,
            propagation: None,
            read_only_force_recursive: None,
            read_only_non_recursive: None,
        }
    }
}

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

/// InspectMount : InspectMount provides a record of a single mount in a container. It contains fields for both named and normal volumes. Only user-specified volumes will be included, and tmpfs volumes are not included even if the user specified them.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct InspectMount {
    /// The destination directory for the volume. Specified as a path within the container, as it would be passed into the OCI runtime.
    #[serde(rename = "Destination", skip_serializing_if = "Option::is_none")]
    pub destination: Option<String>,
    /// The driver used for the named volume. Empty for bind mounts.
    #[serde(rename = "Driver", skip_serializing_if = "Option::is_none")]
    pub driver: Option<String>,
    /// Contains SELinux :z/:Z mount options. Unclear what, if anything, else goes in here.
    #[serde(rename = "Mode", skip_serializing_if = "Option::is_none")]
    pub mode: Option<String>,
    /// The name of the volume. Empty for bind mounts.
    #[serde(rename = "Name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// All remaining mount options. Additional data, not present in the original output.
    #[serde(rename = "Options", skip_serializing_if = "Option::is_none")]
    pub options: Option<Vec<String>>,
    /// Mount propagation for the mount. Can be empty if not specified, but is always printed - no omitempty.
    #[serde(rename = "Propagation", skip_serializing_if = "Option::is_none")]
    pub propagation: Option<String>,
    /// Whether the volume is read-write
    #[serde(rename = "RW", skip_serializing_if = "Option::is_none")]
    pub rw: Option<bool>,
    /// The source directory for the volume.
    #[serde(rename = "Source", skip_serializing_if = "Option::is_none")]
    pub source: Option<String>,
    /// Whether the mount is a volume or bind mount. Allowed values are \"volume\" and \"bind\".
    #[serde(rename = "Type", skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
}

impl InspectMount {
    /// InspectMount provides a record of a single mount in a container. It contains fields for both named and normal volumes. Only user-specified volumes will be included, and tmpfs volumes are not included even if the user specified them.
    pub fn new() -> InspectMount {
        InspectMount {
            destination: None,
            driver: None,
            mode: None,
            name: None,
            options: None,
            propagation: None,
            rw: None,
            source: None,
            r#type: None,
        }
    }
}

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
pub struct PortMapping {
    /// ContainerPort is the port number that will be exposed from the container. Mandatory.
    #[serde(rename = "container_port", skip_serializing_if = "Option::is_none")]
    pub container_port: Option<i32>,
    /// HostIP is the IP that we will bind to on the host. If unset, assumed to be 0.0.0.0 (all interfaces).
    #[serde(rename = "host_ip", skip_serializing_if = "Option::is_none")]
    pub host_ip: Option<String>,
    /// HostPort is the port number that will be forwarded from the host into the container. If omitted, a random port on the host (guaranteed to be over 1024) will be assigned.
    #[serde(rename = "host_port", skip_serializing_if = "Option::is_none")]
    pub host_port: Option<i32>,
    /// Protocol is the protocol forward. Must be either \"tcp\", \"udp\", and \"sctp\", or some combination of these separated by commas. If unset, assumed to be TCP.
    #[serde(rename = "protocol", skip_serializing_if = "Option::is_none")]
    pub protocol: Option<String>,
    /// Range is the number of ports that will be forwarded, starting at HostPort and ContainerPort and counting up. This is 1-indexed, so 1 is assumed to be a single port (only the Hostport:Containerport mapping will be added), 2 is two ports (both Hostport:Containerport and Hostport+1:Containerport+1), etc. If unset, assumed to be 1 (a single port). Both hostport + range and containerport + range must be less than 65536.
    #[serde(rename = "range", skip_serializing_if = "Option::is_none")]
    pub range: Option<i32>,
}

impl PortMapping {
    pub fn new() -> PortMapping {
        PortMapping {
            container_port: None,
            host_ip: None,
            host_port: None,
            protocol: None,
            range: None,
        }
    }
}

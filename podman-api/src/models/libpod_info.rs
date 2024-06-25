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

/// LibpodInfo : Info is the overall struct that describes the host system running libpod/podman
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct LibpodInfo {
    #[serde(rename = "host", skip_serializing_if = "Option::is_none")]
    pub host: Option<Box<models::HostInfo>>,
    #[serde(rename = "plugins", skip_serializing_if = "Option::is_none")]
    pub plugins: Option<Box<models::Plugins>>,
    #[serde(rename = "registries", skip_serializing_if = "Option::is_none")]
    pub registries: Option<std::collections::HashMap<String, serde_json::Value>>,
    #[serde(rename = "store", skip_serializing_if = "Option::is_none")]
    pub store: Option<Box<models::StoreInfo>>,
    #[serde(rename = "version", skip_serializing_if = "Option::is_none")]
    pub version: Option<Box<models::Version>>,
}

impl LibpodInfo {
    /// Info is the overall struct that describes the host system running libpod/podman
    pub fn new() -> LibpodInfo {
        LibpodInfo {
            host: None,
            plugins: None,
            registries: None,
            store: None,
            version: None,
        }
    }
}


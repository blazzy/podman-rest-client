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

/// EndpointSettings : EndpointSettings stores the network endpoint details
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct EndpointSettings {
    #[serde(rename = "Aliases", skip_serializing_if = "Option::is_none")]
    pub aliases: Option<Vec<String>>,
    /// DNSNames holds all the (non fully qualified) DNS names associated to this endpoint. First entry is used to generate PTR records.
    #[serde(rename = "DNSNames", skip_serializing_if = "Option::is_none")]
    pub dns_names: Option<Vec<String>>,
    #[serde(rename = "DriverOpts", skip_serializing_if = "Option::is_none")]
    pub driver_opts: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "EndpointID", skip_serializing_if = "Option::is_none")]
    pub endpoint_id: Option<String>,
    #[serde(rename = "Gateway", skip_serializing_if = "Option::is_none")]
    pub gateway: Option<String>,
    #[serde(rename = "GlobalIPv6Address", skip_serializing_if = "Option::is_none")]
    pub global_ipv6_address: Option<String>,
    #[serde(
        rename = "GlobalIPv6PrefixLen",
        skip_serializing_if = "Option::is_none"
    )]
    pub global_ipv6_prefix_len: Option<i64>,
    #[serde(rename = "IPAMConfig", skip_serializing_if = "Option::is_none")]
    pub ipam_config: Option<Box<models::EndpointIpamConfig>>,
    #[serde(rename = "IPAddress", skip_serializing_if = "Option::is_none")]
    pub ip_address: Option<String>,
    #[serde(rename = "IPPrefixLen", skip_serializing_if = "Option::is_none")]
    pub ip_prefix_len: Option<i64>,
    #[serde(rename = "IPv6Gateway", skip_serializing_if = "Option::is_none")]
    pub ipv6_gateway: Option<String>,
    #[serde(rename = "Links", skip_serializing_if = "Option::is_none")]
    pub links: Option<Vec<String>>,
    /// MacAddress may be used to specify a MAC address when the container is created. Once the container is running, it becomes operational data (it may contain a generated address).
    #[serde(rename = "MacAddress", skip_serializing_if = "Option::is_none")]
    pub mac_address: Option<String>,
    /// Operational data
    #[serde(rename = "NetworkID", skip_serializing_if = "Option::is_none")]
    pub network_id: Option<String>,
}

impl EndpointSettings {
    /// EndpointSettings stores the network endpoint details
    pub fn new() -> EndpointSettings {
        EndpointSettings {
            aliases: None,
            dns_names: None,
            driver_opts: None,
            endpoint_id: None,
            gateway: None,
            global_ipv6_address: None,
            global_ipv6_prefix_len: None,
            ipam_config: None,
            ip_address: None,
            ip_prefix_len: None,
            ipv6_gateway: None,
            links: None,
            mac_address: None,
            network_id: None,
        }
    }
}

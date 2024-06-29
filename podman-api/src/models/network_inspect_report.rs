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
pub struct NetworkInspectReport {
    #[serde(rename = "containers", skip_serializing_if = "Option::is_none")]
    pub containers: Option<std::collections::HashMap<String, models::NetworkContainerInfo>>,
    /// Created contains the timestamp when this network was created.
    #[serde(rename = "created", skip_serializing_if = "Option::is_none")]
    pub created: Option<String>,
    /// DNSEnabled is whether name resolution is active for container on this Network. Only supported with the bridge driver.
    #[serde(rename = "dns_enabled", skip_serializing_if = "Option::is_none")]
    pub dns_enabled: Option<bool>,
    /// Driver for this Network, e.g. bridge, macvlan...
    #[serde(rename = "driver", skip_serializing_if = "Option::is_none")]
    pub driver: Option<String>,
    /// ID of the Network.
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// Internal is whether the Network should not have external routes to public or other Networks.
    #[serde(rename = "internal", skip_serializing_if = "Option::is_none")]
    pub internal: Option<bool>,
    /// IPAMOptions contains options used for the ip assignment.
    #[serde(rename = "ipam_options", skip_serializing_if = "Option::is_none")]
    pub ipam_options: Option<std::collections::HashMap<String, String>>,
    /// IPv6Enabled if set to true an ipv6 subnet should be created for this net.
    #[serde(rename = "ipv6_enabled", skip_serializing_if = "Option::is_none")]
    pub ipv6_enabled: Option<bool>,
    /// Labels is a set of key-value labels that have been applied to the Network.
    #[serde(rename = "labels", skip_serializing_if = "Option::is_none")]
    pub labels: Option<std::collections::HashMap<String, String>>,
    /// Name of the Network.
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// List of custom DNS server for podman's DNS resolver at network level, all the containers attached to this network will consider resolvers configured at network level.
    #[serde(
        rename = "network_dns_servers",
        skip_serializing_if = "Option::is_none"
    )]
    pub network_dns_servers: Option<Vec<String>>,
    /// NetworkInterface is the network interface name on the host.
    #[serde(rename = "network_interface", skip_serializing_if = "Option::is_none")]
    pub network_interface: Option<String>,
    /// Options is a set of key-value options that have been applied to the Network.
    #[serde(rename = "options", skip_serializing_if = "Option::is_none")]
    pub options: Option<std::collections::HashMap<String, String>>,
    /// Routes to use for this network.
    #[serde(rename = "routes", skip_serializing_if = "Option::is_none")]
    pub routes: Option<Vec<models::Route>>,
    /// Subnets to use for this network.
    #[serde(rename = "subnets", skip_serializing_if = "Option::is_none")]
    pub subnets: Option<Vec<models::Subnet>>,
}

impl NetworkInspectReport {
    pub fn new() -> NetworkInspectReport {
        NetworkInspectReport {
            containers: None,
            created: None,
            dns_enabled: None,
            driver: None,
            id: None,
            internal: None,
            ipam_options: None,
            ipv6_enabled: None,
            labels: None,
            name: None,
            network_dns_servers: None,
            network_interface: None,
            options: None,
            routes: None,
            subnets: None,
        }
    }
}

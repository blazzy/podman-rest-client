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
pub struct ImageData {
    #[serde(rename = "Annotations", skip_serializing_if = "Option::is_none")]
    pub annotations: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "Architecture", skip_serializing_if = "Option::is_none")]
    pub architecture: Option<String>,
    #[serde(rename = "Author", skip_serializing_if = "Option::is_none")]
    pub author: Option<String>,
    #[serde(rename = "Comment", skip_serializing_if = "Option::is_none")]
    pub comment: Option<String>,
    #[serde(rename = "Config", skip_serializing_if = "Option::is_none")]
    pub config: Option<Box<models::ImageConfig>>,
    #[serde(rename = "Created", skip_serializing_if = "Option::is_none")]
    pub created: Option<String>,
    /// The following is an example of the contents of Digest types:  sha256:7173b809ca12ec5dee4506cd86be934c4596dd234ee82c0662eac04a8c2c71dc  This allows to abstract the digest behind this type and work only in those terms.
    #[serde(rename = "Digest", skip_serializing_if = "Option::is_none")]
    pub digest: Option<String>,
    #[serde(rename = "GraphDriver", skip_serializing_if = "Option::is_none")]
    pub graph_driver: Option<Box<models::DriverData>>,
    #[serde(rename = "Healthcheck", skip_serializing_if = "Option::is_none")]
    pub healthcheck: Option<Box<models::Schema2HealthConfig>>,
    #[serde(rename = "History", skip_serializing_if = "Option::is_none")]
    pub history: Option<Vec<models::History>>,
    #[serde(rename = "Id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "Labels", skip_serializing_if = "Option::is_none")]
    pub labels: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "ManifestType", skip_serializing_if = "Option::is_none")]
    pub manifest_type: Option<String>,
    #[serde(rename = "NamesHistory", skip_serializing_if = "Option::is_none")]
    pub names_history: Option<Vec<String>>,
    #[serde(rename = "Os", skip_serializing_if = "Option::is_none")]
    pub os: Option<String>,
    #[serde(rename = "Parent", skip_serializing_if = "Option::is_none")]
    pub parent: Option<String>,
    #[serde(rename = "RepoDigests", skip_serializing_if = "Option::is_none")]
    pub repo_digests: Option<Vec<String>>,
    #[serde(rename = "RepoTags", skip_serializing_if = "Option::is_none")]
    pub repo_tags: Option<Vec<String>>,
    #[serde(rename = "RootFS", skip_serializing_if = "Option::is_none")]
    pub root_fs: Option<Box<models::RootFs>>,
    #[serde(rename = "Size", skip_serializing_if = "Option::is_none")]
    pub size: Option<i64>,
    #[serde(rename = "User", skip_serializing_if = "Option::is_none")]
    pub user: Option<String>,
    #[serde(rename = "Version", skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
    #[serde(rename = "VirtualSize", skip_serializing_if = "Option::is_none")]
    pub virtual_size: Option<i64>,
}

impl ImageData {
    pub fn new() -> ImageData {
        ImageData {
            annotations: None,
            architecture: None,
            author: None,
            comment: None,
            config: None,
            created: None,
            digest: None,
            graph_driver: None,
            healthcheck: None,
            history: None,
            id: None,
            labels: None,
            manifest_type: None,
            names_history: None,
            os: None,
            parent: None,
            repo_digests: None,
            repo_tags: None,
            root_fs: None,
            size: None,
            user: None,
            version: None,
            virtual_size: None,
        }
    }
}
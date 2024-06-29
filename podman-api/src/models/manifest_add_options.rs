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

/// ManifestAddOptions : ManifestAddOptions provides model for adding digests to manifest list
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct ManifestAddOptions {
    /// True when operating on a list to include all images
    #[serde(rename = "all", skip_serializing_if = "Option::is_none")]
    pub all: Option<bool>,
    /// Annotation to add to the item in the manifest list
    #[serde(rename = "annotation", skip_serializing_if = "Option::is_none")]
    pub annotation: Option<Vec<String>>,
    /// Annotations to add to the item in the manifest list by a map which is preferred over Annotation
    #[serde(rename = "annotations", skip_serializing_if = "Option::is_none")]
    pub annotations: Option<std::collections::HashMap<String, String>>,
    /// Arch overrides the architecture for the item in the manifest list
    #[serde(rename = "arch", skip_serializing_if = "Option::is_none")]
    pub arch: Option<String>,
    /// Feature list for the item in the manifest list
    #[serde(rename = "features", skip_serializing_if = "Option::is_none")]
    pub features: Option<Vec<String>>,
    /// Images is an optional list of image references to add to manifest list
    #[serde(rename = "images", skip_serializing_if = "Option::is_none")]
    pub images: Option<Vec<String>>,
    /// IndexAnnotation is a slice of key=value annotations to add to the manifest list itself
    #[serde(rename = "index_annotation", skip_serializing_if = "Option::is_none")]
    pub index_annotation: Option<Vec<String>>,
    /// IndexAnnotations is a map of key:value annotations to add to the manifest list itself, by a map which is preferred over IndexAnnotation
    #[serde(rename = "index_annotations", skip_serializing_if = "Option::is_none")]
    pub index_annotations: Option<std::collections::HashMap<String, String>>,
    /// OS overrides the operating system for the item in the manifest list
    #[serde(rename = "os", skip_serializing_if = "Option::is_none")]
    pub os: Option<String>,
    /// OS features for the item in the manifest list
    #[serde(rename = "os_features", skip_serializing_if = "Option::is_none")]
    pub os_features: Option<Vec<String>>,
    /// OSVersion overrides the operating system for the item in the manifest list
    #[serde(rename = "os_version", skip_serializing_if = "Option::is_none")]
    pub os_version: Option<String>,
    /// IndexSubject is a subject value to set in the manifest list itself
    #[serde(rename = "subject", skip_serializing_if = "Option::is_none")]
    pub subject: Option<String>,
    /// Variant for the item in the manifest list
    #[serde(rename = "variant", skip_serializing_if = "Option::is_none")]
    pub variant: Option<String>,
}

impl ManifestAddOptions {
    /// ManifestAddOptions provides model for adding digests to manifest list
    pub fn new() -> ManifestAddOptions {
        ManifestAddOptions {
            all: None,
            annotation: None,
            annotations: None,
            arch: None,
            features: None,
            images: None,
            index_annotation: None,
            index_annotations: None,
            os: None,
            os_features: None,
            os_version: None,
            subject: None,
            variant: None,
        }
    }
}

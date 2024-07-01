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

/// ContainerSecurityConfig : ContainerSecurityConfig is a container's security features, including SELinux, Apparmor, and Seccomp.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct ContainerSecurityConfig {
    /// ApparmorProfile is the name of the Apparmor profile the container will use. Optional.
    #[serde(rename = "apparmor_profile", skip_serializing_if = "Option::is_none")]
    pub apparmor_profile: Option<String>,
    /// CapAdd are capabilities which will be added to the container. Conflicts with Privileged. Optional.
    #[serde(rename = "cap_add", skip_serializing_if = "Option::is_none")]
    pub cap_add: Option<Vec<String>>,
    /// CapDrop are capabilities which will be removed from the container. Conflicts with Privileged. Optional.
    #[serde(rename = "cap_drop", skip_serializing_if = "Option::is_none")]
    pub cap_drop: Option<Vec<String>>,
    /// Groups are a list of supplemental groups the container's user will be granted access to. Optional.
    #[serde(rename = "groups", skip_serializing_if = "Option::is_none")]
    pub groups: Option<Vec<String>>,
    #[serde(rename = "idmappings", skip_serializing_if = "Option::is_none")]
    pub idmappings: Option<Box<models::IdMappingOptions>>,
    /// LabelNested indicates whether or not the container is allowed to run fully nested containers including SELinux labelling. Optional.
    #[serde(rename = "label_nested", skip_serializing_if = "Option::is_none")]
    pub label_nested: Option<bool>,
    /// Mask is the path we want to mask in the container. This masks the paths given in addition to the default list. Optional
    #[serde(rename = "mask", skip_serializing_if = "Option::is_none")]
    pub mask: Option<Vec<String>>,
    /// NoNewPrivileges is whether the container will set the no new privileges flag on create, which disables gaining additional privileges (e.g. via setuid) in the container. Optional.
    #[serde(rename = "no_new_privileges", skip_serializing_if = "Option::is_none")]
    pub no_new_privileges: Option<bool>,
    /// Privileged is whether the container is privileged. Privileged does the following: Adds all devices on the system to the container. Adds all capabilities to the container. Disables Seccomp, SELinux, and Apparmor confinement. (Though SELinux can be manually re-enabled). TODO: this conflicts with things. TODO: this does more. Optional.
    #[serde(rename = "privileged", skip_serializing_if = "Option::is_none")]
    pub privileged: Option<bool>,
    /// ProcOpts are the options used for the proc mount.
    #[serde(rename = "procfs_opts", skip_serializing_if = "Option::is_none")]
    pub procfs_opts: Option<Vec<String>>,
    /// ReadOnlyFilesystem indicates that everything will be mounted as read-only. Optional.
    #[serde(
        rename = "read_only_filesystem",
        skip_serializing_if = "Option::is_none"
    )]
    pub read_only_filesystem: Option<bool>,
    /// ReadWriteTmpfs indicates that when running with a ReadOnlyFilesystem mount temporary file systems. Optional.
    #[serde(rename = "read_write_tmpfs", skip_serializing_if = "Option::is_none")]
    pub read_write_tmpfs: Option<bool>,
    /// SeccompPolicy determines which seccomp profile gets applied the container. valid values: empty,default,image
    #[serde(rename = "seccomp_policy", skip_serializing_if = "Option::is_none")]
    pub seccomp_policy: Option<String>,
    /// SeccompProfilePath is the path to a JSON file containing the container's Seccomp profile. If not specified, no Seccomp profile will be used. Optional.
    #[serde(
        rename = "seccomp_profile_path",
        skip_serializing_if = "Option::is_none"
    )]
    pub seccomp_profile_path: Option<String>,
    /// SelinuxProcessLabel is the process label the container will use. If SELinux is enabled and this is not specified, a label will be automatically generated if not specified. Optional.
    #[serde(rename = "selinux_opts", skip_serializing_if = "Option::is_none")]
    pub selinux_opts: Option<Vec<String>>,
    /// Umask is the umask the init process of the container will be run with.
    #[serde(rename = "umask", skip_serializing_if = "Option::is_none")]
    pub umask: Option<String>,
    /// Unmask a path in the container. Some paths are masked by default, preventing them from being accessed within the container; this undoes that masking. If ALL is passed, all paths will be unmasked. Optional.
    #[serde(rename = "unmask", skip_serializing_if = "Option::is_none")]
    pub unmask: Option<Vec<String>>,
    /// User is the user the container will be run as. Can be given as a UID or a username; if a username, it will be resolved within the container, using the container's /etc/passwd. If unset, the container will be run as root. Optional.
    #[serde(rename = "user", skip_serializing_if = "Option::is_none")]
    pub user: Option<String>,
    #[serde(rename = "userns", skip_serializing_if = "Option::is_none")]
    pub userns: Option<Box<models::Namespace>>,
}

impl ContainerSecurityConfig {
    /// ContainerSecurityConfig is a container's security features, including SELinux, Apparmor, and Seccomp.
    pub fn new() -> ContainerSecurityConfig {
        ContainerSecurityConfig {
            apparmor_profile: None,
            cap_add: None,
            cap_drop: None,
            groups: None,
            idmappings: None,
            label_nested: None,
            mask: None,
            no_new_privileges: None,
            privileged: None,
            procfs_opts: None,
            read_only_filesystem: None,
            read_write_tmpfs: None,
            seccomp_policy: None,
            seccomp_profile_path: None,
            selinux_opts: None,
            umask: None,
            unmask: None,
            user: None,
            userns: None,
        }
    }
}
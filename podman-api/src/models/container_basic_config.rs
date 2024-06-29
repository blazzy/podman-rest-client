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
pub struct ContainerBasicConfig {
    /// Annotations are key-value options passed into the container runtime that can be used to trigger special behavior. Optional.
    #[serde(rename = "annotations", skip_serializing_if = "Option::is_none")]
    pub annotations: Option<std::collections::HashMap<String, String>>,
    /// Command is the container's command. If not given and Image is specified, this will be populated by the image's configuration. Optional.
    #[serde(rename = "command", skip_serializing_if = "Option::is_none")]
    pub command: Option<Vec<String>>,
    /// ConmonPidFile is a path at which a PID file for Conmon will be placed. If not given, a default location will be used. Optional.
    #[serde(rename = "conmon_pid_file", skip_serializing_if = "Option::is_none")]
    pub conmon_pid_file: Option<String>,
    /// ContainerCreateCommand is the command that was used to create this container. This will be shown in the output of Inspect() on the container, and may also be used by some tools that wish to recreate the container (e.g. `podman generate systemd --new`). Optional.
    #[serde(
        rename = "containerCreateCommand",
        skip_serializing_if = "Option::is_none"
    )]
    pub container_create_command: Option<Vec<String>>,
    /// DependencyContainers is an array of containers this container depends on. Dependency containers must be started before this container. Dependencies can be specified by name or full/partial ID. Optional.
    #[serde(
        rename = "dependencyContainers",
        skip_serializing_if = "Option::is_none"
    )]
    pub dependency_containers: Option<Vec<String>>,
    /// Entrypoint is the container's entrypoint. If not given and Image is specified, this will be populated by the image's configuration. Optional.
    #[serde(rename = "entrypoint", skip_serializing_if = "Option::is_none")]
    pub entrypoint: Option<Vec<String>>,
    /// Env is a set of environment variables that will be set in the container. Optional.
    #[serde(rename = "env", skip_serializing_if = "Option::is_none")]
    pub env: Option<std::collections::HashMap<String, String>>,
    /// EnvHost indicates that the host environment should be added to container Optional.
    #[serde(rename = "env_host", skip_serializing_if = "Option::is_none")]
    pub env_host: Option<bool>,
    /// EnvMerge takes the specified environment variables from image and preprocess them before injecting them into the container. Optional.
    #[serde(rename = "envmerge", skip_serializing_if = "Option::is_none")]
    pub envmerge: Option<Vec<String>>,
    /// GroupEntry specifies an arbitrary string to append to the container's /etc/group file. Optional.
    #[serde(rename = "group_entry", skip_serializing_if = "Option::is_none")]
    pub group_entry: Option<String>,
    /// Hostname is the container's hostname. If not set, the hostname will not be modified (if UtsNS is not private) or will be set to the container ID (if UtsNS is private). Conflicts with UtsNS if UtsNS is not set to private. Optional.
    #[serde(rename = "hostname", skip_serializing_if = "Option::is_none")]
    pub hostname: Option<String>,
    /// HostUsers is a list of host usernames or UIDs to add to the container etc/passwd file
    #[serde(rename = "hostusers", skip_serializing_if = "Option::is_none")]
    pub hostusers: Option<Vec<String>>,
    /// EnvHTTPProxy indicates that the http host proxy environment variables should be added to container Optional.
    #[serde(rename = "httpproxy", skip_serializing_if = "Option::is_none")]
    pub httpproxy: Option<bool>,
    /// InitContainerType describes if this container is an init container and if so, what type: always or once. Optional.
    #[serde(
        rename = "init_container_type",
        skip_serializing_if = "Option::is_none"
    )]
    pub init_container_type: Option<String>,
    /// Labels are key-value pairs that are used to add metadata to containers. Optional.
    #[serde(rename = "labels", skip_serializing_if = "Option::is_none")]
    pub labels: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "log_configuration", skip_serializing_if = "Option::is_none")]
    pub log_configuration: Option<Box<models::LogConfigLibpod>>,
    /// Passwd is a container run option that determines if we are validating users/groups before running the container
    #[serde(rename = "manage_password", skip_serializing_if = "Option::is_none")]
    pub manage_password: Option<bool>,
    /// Name is the name the container will be given. If no name is provided, one will be randomly generated. Optional.
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// OCIRuntime is the name of the OCI runtime that will be used to create the container. If not specified, the default will be used. Optional.
    #[serde(rename = "oci_runtime", skip_serializing_if = "Option::is_none")]
    pub oci_runtime: Option<String>,
    /// PasswdEntry specifies an arbitrary string to append to the container's /etc/passwd file. Optional.
    #[serde(rename = "passwd_entry", skip_serializing_if = "Option::is_none")]
    pub passwd_entry: Option<String>,
    #[serde(rename = "personality", skip_serializing_if = "Option::is_none")]
    pub personality: Option<Box<models::LinuxPersonality>>,
    #[serde(rename = "pidns", skip_serializing_if = "Option::is_none")]
    pub pidns: Option<Box<models::Namespace>>,
    /// Pod is the ID of the pod the container will join. Optional.
    #[serde(rename = "pod", skip_serializing_if = "Option::is_none")]
    pub pod: Option<String>,
    /// Remove indicates if the container should be removed once it has been started and exits. Optional.
    #[serde(rename = "remove", skip_serializing_if = "Option::is_none")]
    pub remove: Option<bool>,
    /// RestartPolicy is the container's restart policy - an action which will be taken when the container exits. If not given, the default policy, which does nothing, will be used. Optional.
    #[serde(rename = "restart_policy", skip_serializing_if = "Option::is_none")]
    pub restart_policy: Option<String>,
    /// RestartRetries is the number of attempts that will be made to restart the container. Only available when RestartPolicy is set to \"on-failure\". Optional.
    #[serde(rename = "restart_tries", skip_serializing_if = "Option::is_none")]
    pub restart_tries: Option<i32>,
    /// Determine how to handle the NOTIFY_SOCKET - do we participate or pass it through \"container\" - let the OCI runtime deal with it, advertise conmon's MAINPID \"conmon-only\" - advertise conmon's MAINPID, send READY when started, don't pass to OCI \"ignore\" - unset NOTIFY_SOCKET Optional.
    #[serde(rename = "sdnotifyMode", skip_serializing_if = "Option::is_none")]
    pub sdnotify_mode: Option<String>,
    /// EnvSecrets are secrets that will be set as environment variables Optional.
    #[serde(rename = "secret_env", skip_serializing_if = "Option::is_none")]
    pub secret_env: Option<std::collections::HashMap<String, String>>,
    /// Stdin is whether the container will keep its STDIN open. Optional.
    #[serde(rename = "stdin", skip_serializing_if = "Option::is_none")]
    pub stdin: Option<bool>,
    /// It implements the os.Signal interface.
    #[serde(rename = "stop_signal", skip_serializing_if = "Option::is_none")]
    pub stop_signal: Option<i64>,
    /// StopTimeout is a timeout between the container's stop signal being sent and SIGKILL being sent. If not provided, the default will be used. If 0 is used, stop signal will not be sent, and SIGKILL will be sent instead. Optional.
    #[serde(rename = "stop_timeout", skip_serializing_if = "Option::is_none")]
    pub stop_timeout: Option<i32>,
    /// Sysctl sets kernel parameters for the container
    #[serde(rename = "sysctl", skip_serializing_if = "Option::is_none")]
    pub sysctl: Option<std::collections::HashMap<String, String>>,
    /// Systemd is whether the container will be started in systemd mode. Valid options are \"true\", \"false\", and \"always\". \"true\" enables this mode only if the binary run in the container is sbin/init or systemd. \"always\" unconditionally enables systemd mode. \"false\" unconditionally disables systemd mode. If enabled, mounts and stop signal will be modified. If set to \"always\" or set to \"true\" and conditionally triggered, conflicts with StopSignal. If not specified, \"false\" will be assumed. Optional.
    #[serde(rename = "systemd", skip_serializing_if = "Option::is_none")]
    pub systemd: Option<String>,
    /// Terminal is whether the container will create a PTY. Optional.
    #[serde(rename = "terminal", skip_serializing_if = "Option::is_none")]
    pub terminal: Option<bool>,
    /// Timeout is a maximum time in seconds the container will run before main process is sent SIGKILL. If 0 is used, signal will not be sent. Container can run indefinitely if they do not stop after the default termination signal. Optional.
    #[serde(rename = "timeout", skip_serializing_if = "Option::is_none")]
    pub timeout: Option<i32>,
    /// Timezone is the timezone inside the container. Local means it has the same timezone as the host machine Optional.
    #[serde(rename = "timezone", skip_serializing_if = "Option::is_none")]
    pub timezone: Option<String>,
    /// UnsetEnv unsets the specified default environment variables from the image or from buildin or containers.conf Optional.
    #[serde(rename = "unsetenv", skip_serializing_if = "Option::is_none")]
    pub unsetenv: Option<Vec<String>>,
    /// UnsetEnvAll unsetall default environment variables from the image or from buildin or containers.conf UnsetEnvAll unsets all default environment variables from the image or from buildin Optional.
    #[serde(rename = "unsetenvall", skip_serializing_if = "Option::is_none")]
    pub unsetenvall: Option<bool>,
    #[serde(rename = "utsns", skip_serializing_if = "Option::is_none")]
    pub utsns: Option<Box<models::Namespace>>,
}

impl ContainerBasicConfig {
    pub fn new() -> ContainerBasicConfig {
        ContainerBasicConfig {
            annotations: None,
            command: None,
            conmon_pid_file: None,
            container_create_command: None,
            dependency_containers: None,
            entrypoint: None,
            env: None,
            env_host: None,
            envmerge: None,
            group_entry: None,
            hostname: None,
            hostusers: None,
            httpproxy: None,
            init_container_type: None,
            labels: None,
            log_configuration: None,
            manage_password: None,
            name: None,
            oci_runtime: None,
            passwd_entry: None,
            personality: None,
            pidns: None,
            pod: None,
            remove: None,
            restart_policy: None,
            restart_tries: None,
            sdnotify_mode: None,
            secret_env: None,
            stdin: None,
            stop_signal: None,
            stop_timeout: None,
            sysctl: None,
            systemd: None,
            terminal: None,
            timeout: None,
            timezone: None,
            unsetenv: None,
            unsetenvall: None,
            utsns: None,
        }
    }
}

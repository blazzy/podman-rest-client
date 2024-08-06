use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Serialize, Deserialize)]
/// ContainerBasicConfig contains the basic parts of a container.
pub struct ContainerBasicConfig {
    /// Annotations are key-value options passed into the container runtime
    /// that can be used to trigger special behavior.
    /// Optional.
    pub annotations: Option<std::collections::HashMap<String, String>>,

    /// Command is the container's command.
    /// If not given and Image is specified, this will be populated by the
    /// image's configuration.
    /// Optional.
    pub command: Option<Vec<String>>,

    /// ConmonPidFile is a path at which a PID file for Conmon will be
    /// placed.
    /// If not given, a default location will be used.
    /// Optional.
    pub conmon_pid_file: Option<String>,

    /// ContainerCreateCommand is the command that was used to create this
    /// container.
    /// This will be shown in the output of Inspect() on the container, and
    /// may also be used by some tools that wish to recreate the container
    /// (e.g. `podman generate systemd --new`).
    /// Optional.
    #[serde(rename = "containerCreateCommand")]
    pub container_create_command: Option<Vec<String>>,

    /// DependencyContainers is an array of containers this container
    /// depends on. Dependency containers must be started before this
    /// container. Dependencies can be specified by name or full/partial ID.
    /// Optional.
    #[serde(rename = "dependencyContainers")]
    pub dependency_containers: Option<Vec<String>>,

    /// Entrypoint is the container's entrypoint.
    /// If not given and Image is specified, this will be populated by the
    /// image's configuration.
    /// Optional.
    pub entrypoint: Option<Vec<String>>,

    /// Env is a set of environment variables that will be set in the
    /// container.
    /// Optional.
    pub env: Option<std::collections::HashMap<String, String>>,

    /// EnvHost indicates that the host environment should be added to container
    /// Optional.
    pub env_host: Option<bool>,

    /// EnvMerge takes the specified environment variables from image and preprocess them before injecting them into the
    /// container.
    /// Optional.
    pub envmerge: Option<Vec<String>>,

    /// GroupEntry specifies an arbitrary string to append to the container's /etc/group file.
    /// Optional.
    pub group_entry: Option<String>,

    /// Hostname is the container's hostname. If not set, the hostname will
    /// not be modified (if UtsNS is not private) or will be set to the
    /// container ID (if UtsNS is private).
    /// Conflicts with UtsNS if UtsNS is not set to private.
    /// Optional.
    pub hostname: Option<String>,

    /// HostUsers is a list of host usernames or UIDs to add to the container
    /// etc/passwd file
    pub hostusers: Option<Vec<String>>,

    /// EnvHTTPProxy indicates that the http host proxy environment variables
    /// should be added to container
    /// Optional.
    pub httpproxy: Option<bool>,

    /// InitContainerType describes if this container is an init container
    /// and if so, what type: always or once.
    /// Optional.
    pub init_container_type: Option<String>,

    /// Labels are key-value pairs that are used to add metadata to
    /// containers.
    /// Optional.
    pub labels: Option<std::collections::HashMap<String, String>>,

    pub log_configuration: Option<super::super::models::LogConfigLibpod>,

    /// Passwd is a container run option that determines if we are validating users/groups before running the container
    pub manage_password: Option<bool>,

    /// Name is the name the container will be given.
    /// If no name is provided, one will be randomly generated.
    /// Optional.
    pub name: Option<String>,

    /// OCIRuntime is the name of the OCI runtime that will be used to create
    /// the container.
    /// If not specified, the default will be used.
    /// Optional.
    pub oci_runtime: Option<String>,

    /// PasswdEntry specifies an arbitrary string to append to the container's /etc/passwd file.
    /// Optional.
    pub passwd_entry: Option<String>,

    pub personality: Option<super::super::models::LinuxPersonality>,

    pub pidns: Option<super::super::models::Namespace>,

    /// Pod is the ID of the pod the container will join.
    /// Optional.
    pub pod: Option<String>,

    /// Remove indicates if the container should be removed once it has been started
    /// and exits.
    /// Optional.
    pub remove: Option<bool>,

    /// RestartPolicy is the container's restart policy - an action which
    /// will be taken when the container exits.
    /// If not given, the default policy, which does nothing, will be used.
    /// Optional.
    pub restart_policy: Option<String>,

    /// RestartRetries is the number of attempts that will be made to restart
    /// the container.
    /// Only available when RestartPolicy is set to "on-failure".
    /// Optional.
    pub restart_tries: Option<u64>,

    /// Determine how to handle the NOTIFY_SOCKET - do we participate or pass it through
    /// "container" - let the OCI runtime deal with it, advertise conmon's MAINPID
    /// "conmon-only" - advertise conmon's MAINPID, send READY when started, don't pass to OCI
    /// "ignore" - unset NOTIFY_SOCKET
    /// Optional.
    #[serde(rename = "sdnotifyMode")]
    pub sdnotify_mode: Option<String>,

    /// EnvSecrets are secrets that will be set as environment variables
    /// Optional.
    pub secret_env: Option<std::collections::HashMap<String, String>>,

    /// Stdin is whether the container will keep its STDIN open.
    /// Optional.
    pub stdin: Option<bool>,

    pub stop_signal: Option<i64>,

    /// StopTimeout is a timeout between the container's stop signal being
    /// sent and SIGKILL being sent.
    /// If not provided, the default will be used.
    /// If 0 is used, stop signal will not be sent, and SIGKILL will be sent
    /// instead.
    /// Optional.
    pub stop_timeout: Option<u64>,

    /// Sysctl sets kernel parameters for the container
    pub sysctl: Option<std::collections::HashMap<String, String>>,

    /// Systemd is whether the container will be started in systemd mode.
    /// Valid options are "true", "false", and "always".
    /// "true" enables this mode only if the binary run in the container is
    /// sbin/init or systemd. "always" unconditionally enables systemd mode.
    /// "false" unconditionally disables systemd mode.
    /// If enabled, mounts and stop signal will be modified.
    /// If set to "always" or set to "true" and conditionally triggered,
    /// conflicts with StopSignal.
    /// If not specified, "false" will be assumed.
    /// Optional.
    pub systemd: Option<String>,

    /// Terminal is whether the container will create a PTY.
    /// Optional.
    pub terminal: Option<bool>,

    /// Timeout is a maximum time in seconds the container will run before
    /// main process is sent SIGKILL.
    /// If 0 is used, signal will not be sent. Container can run indefinitely
    /// if they do not stop after the default termination signal.
    /// Optional.
    pub timeout: Option<u64>,

    /// Timezone is the timezone inside the container.
    /// Local means it has the same timezone as the host machine
    /// Optional.
    pub timezone: Option<String>,

    /// UnsetEnv unsets the specified default environment variables from the image or from buildin or containers.conf
    /// Optional.
    pub unsetenv: Option<Vec<String>>,

    /// UnsetEnvAll unsetall default environment variables from the image or from buildin or containers.conf
    /// UnsetEnvAll unsets all default environment variables from the image or from buildin
    /// Optional.
    pub unsetenvall: Option<bool>,

    pub utsns: Option<super::super::models::Namespace>,
}

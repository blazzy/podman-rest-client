use serde::{Deserialize, Serialize};
#[derive(Default, Debug, Serialize, Deserialize)]
/// SpecGenerator creates an OCI spec and Libpod configuration options to create
/// a container based on the given configuration.
pub struct SpecGenerator {
    /// Map of networks names or ids that the container should join.
    /// You can request additional settings for each network, you can
    /// set network aliases, static ips, static mac address  and the
    /// network interface name for this container on the specific network.
    /// If the map is empty and the bridge network mode is set the container
    /// will be joined to the default network.
    /// Optional.
    #[serde(rename = "Networks")]
    pub networks:
        Option<std::collections::HashMap<String, super::super::models::PerNetworkOptions>>,
    /// Annotations are key-value options passed into the container runtime
    /// that can be used to trigger special behavior.
    /// Optional.
    pub annotations: Option<std::collections::HashMap<String, String>>,
    /// ApparmorProfile is the name of the Apparmor profile the container
    /// will use.
    /// Optional.
    pub apparmor_profile: Option<String>,
    /// BaseHostsFile is the path to a hosts file, the entries from this file
    /// are added to the containers hosts file. As special value "image" is
    /// allowed which uses the /etc/hosts file from within the image and "none"
    /// which uses no base file at all. If it is empty we should default
    /// to the base_hosts_file configuration in containers.conf.
    /// Optional.
    pub base_hosts_file: Option<String>,
    /// CapAdd are capabilities which will be added to the container.
    /// Conflicts with Privileged.
    /// Optional.
    pub cap_add: Option<Vec<String>>,
    /// CapDrop are capabilities which will be removed from the container.
    /// Conflicts with Privileged.
    /// Optional.
    pub cap_drop: Option<Vec<String>>,
    /// CgroupParent is the container's Cgroup parent.
    /// If not set, the default for the current cgroup driver will be used.
    /// Optional.
    pub cgroup_parent: Option<String>,
    pub cgroupns: Option<super::super::models::Namespace>,
    /// CgroupsMode sets a policy for how cgroups will be created for the
    /// container, including the ability to disable creation entirely.
    /// Optional.
    pub cgroups_mode: Option<String>,
    /// ChrootDirs is an additional set of directories that need to be
    /// treated as root directories. Standard bind mounts will be mounted
    /// into paths relative to these directories.
    /// Optional.
    pub chroot_directories: Option<Vec<String>>,
    /// CNINetworks is a list of CNI networks to join the container to.
    /// If this list is empty, the default CNI network will be joined
    /// instead. If at least one entry is present, we will not join the
    /// default network (unless it is part of this list).
    /// Only available if NetNS is set to bridge.
    /// Optional.
    /// Deprecated: as of podman 4.0 use "Networks" instead.
    pub cni_networks: Option<Vec<String>>,
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
    /// Create the working directory if it doesn't exist.
    /// If unset, it doesn't create it.
    /// Optional.
    pub create_working_dir: Option<bool>,
    /// DependencyContainers is an array of containers this container
    /// depends on. Dependency containers must be started before this
    /// container. Dependencies can be specified by name or full/partial ID.
    /// Optional.
    #[serde(rename = "dependencyContainers")]
    pub dependency_containers: Option<Vec<String>>,
    /// DeviceCgroupRule are device cgroup rules that allow containers
    /// to use additional types of devices.
    pub device_cgroup_rule: Option<Vec<super::super::models::LinuxDeviceCgroup>>,
    /// Devices are devices that will be added to the container.
    /// Optional.
    pub devices: Option<Vec<super::super::models::LinuxDevice>>,
    /// DevicesFrom specifies that this container will mount the device(s) from other container(s).
    /// Optional.
    pub devices_from: Option<Vec<String>>,
    /// DNSOptions is a set of DNS options that will be used in the
    /// container's resolv.conf, replacing the host's DNS options which are
    /// used by default.
    /// Conflicts with UseImageResolvConf.
    /// Optional.
    pub dns_option: Option<Vec<String>>,
    /// DNSSearch is a set of DNS search domains that will be used in the
    /// container's resolv.conf, replacing the host's DNS search domains
    /// which are used by default.
    /// Conflicts with UseImageResolvConf.
    /// Optional.
    pub dns_search: Option<Vec<String>>,
    /// DNSServers is a set of DNS servers that will be used in the
    /// container's resolv.conf, replacing the host's DNS Servers which are
    /// used by default.
    /// Conflicts with UseImageResolvConf.
    /// Optional.
    pub dns_server: Option<Vec<String>>,
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
    /// Expose is a number of ports that will be forwarded to the container
    /// if PublishExposedPorts is set.
    /// Expose is a map of uint16 (port number) to a string representing
    /// protocol i.e map[uint16]string. Allowed protocols are "tcp", "udp", and "sctp", or some
    /// combination of the three separated by commas.
    /// If protocol is set to "" we will assume TCP.
    /// Only available if NetNS is set to Bridge or Slirp, and
    /// PublishExposedPorts is set.
    /// Optional.
    pub expose: Option<()>,
    /// GroupEntry specifies an arbitrary string to append to the container's /etc/group file.
    /// Optional.
    pub group_entry: Option<String>,
    /// Groups are a list of supplemental groups the container's user will
    /// be granted access to.
    /// Optional.
    pub groups: Option<Vec<String>>,
    pub health_check_on_failure_action: Option<i64>,
    pub healthconfig: Option<super::super::models::Schema2HealthConfig>,
    /// HostDeviceList is used to recreate the mounted device on inherited containers
    pub host_device_list: Option<Vec<super::super::models::LinuxDevice>>,
    /// HostAdd is a set of hosts which will be added to the container's
    /// etc/hosts file.
    /// Conflicts with UseImageHosts.
    /// Optional.
    pub hostadd: Option<Vec<String>>,
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
    pub idmappings: Option<super::super::models::IdMappingOptions>,
    /// Image is the image the container will be based on. The image will be
    /// used as the container's root filesystem, and its environment vars,
    /// volumes, and other configuration will be applied to the container.
    /// Conflicts with Rootfs.
    /// At least one of Image or Rootfs must be specified.
    pub image: Option<String>,
    /// ImageArch is the user-specified image architecture.
    /// Used to select a different variant from a manifest list.
    /// Optional.
    pub image_arch: Option<String>,
    /// ImageOS is the user-specified OS of the image.
    /// Used to select a different variant from a manifest list.
    /// Optional.
    pub image_os: Option<String>,
    /// ImageVariant is the user-specified image variant.
    /// Used to select a different variant from a manifest list.
    /// Optional.
    pub image_variant: Option<String>,
    /// ImageVolumeMode indicates how image volumes will be created.
    /// Supported modes are "ignore" (do not create), "tmpfs" (create as
    /// tmpfs), and "anonymous" (create as anonymous volumes).
    /// The default if unset is anonymous.
    /// Optional.
    pub image_volume_mode: Option<String>,
    /// Image volumes bind-mount a container-image mount into the container.
    /// Optional.
    pub image_volumes: Option<Vec<super::super::models::ImageVolume>>,
    /// Init specifies that an init binary will be mounted into the
    /// container, and will be used as PID1.
    /// Optional.
    pub init: Option<bool>,
    /// InitContainerType describes if this container is an init container
    /// and if so, what type: always or once.
    /// Optional.
    pub init_container_type: Option<String>,
    /// InitPath specifies the path to the init binary that will be added if
    /// Init is specified above. If not specified, the default set in the
    /// Libpod config will be used. Ignored if Init above is not set.
    /// Optional.
    pub init_path: Option<String>,
    #[serde(rename = "intelRdt")]
    pub intel_rdt: Option<super::super::models::LinuxIntelRdt>,
    pub ipcns: Option<super::super::models::Namespace>,
    /// LabelNested indicates whether or not the container is allowed to
    /// run fully nested containers including SELinux labelling.
    /// Optional.
    pub label_nested: Option<bool>,
    /// Labels are key-value pairs that are used to add metadata to
    /// containers.
    /// Optional.
    pub labels: Option<std::collections::HashMap<String, String>>,
    pub log_configuration: Option<super::super::models::LogConfigLibpod>,
    /// Passwd is a container run option that determines if we are validating users/groups before running the container
    pub manage_password: Option<bool>,
    /// Mask is the path we want to mask in the container. This masks the paths
    /// given in addition to the default list.
    /// Optional
    pub mask: Option<Vec<String>>,
    /// Mounts are mounts that will be added to the container.
    /// These will supersede Image Volumes and VolumesFrom volumes where
    /// there are conflicts.
    /// Optional.
    pub mounts: Option<Vec<super::super::models::Mount>>,
    /// Name is the name the container will be given.
    /// If no name is provided, one will be randomly generated.
    /// Optional.
    pub name: Option<String>,
    pub netns: Option<super::super::models::Namespace>,
    /// NetworkOptions are additional options for each network
    /// Optional.
    pub network_options: Option<std::collections::HashMap<String, Vec<String>>>,
    /// NoNewPrivileges is whether the container will set the no new
    /// privileges flag on create, which disables gaining additional
    /// privileges (e.g. via setuid) in the container.
    /// Optional.
    pub no_new_privileges: Option<bool>,
    /// OCIRuntime is the name of the OCI runtime that will be used to create
    /// the container.
    /// If not specified, the default will be used.
    /// Optional.
    pub oci_runtime: Option<String>,
    /// OOMScoreAdj adjusts the score used by the OOM killer to determine
    /// processes to kill for the container's process.
    /// Optional.
    pub oom_score_adj: Option<i64>,
    /// Overlay volumes are named volumes that will be added to the container.
    /// Optional.
    pub overlay_volumes: Option<Vec<super::super::models::OverlayVolume>>,
    /// PasswdEntry specifies an arbitrary string to append to the container's /etc/passwd file.
    /// Optional.
    pub passwd_entry: Option<String>,
    pub personality: Option<super::super::models::LinuxPersonality>,
    pub pidns: Option<super::super::models::Namespace>,
    /// Pod is the ID of the pod the container will join.
    /// Optional.
    pub pod: Option<String>,
    /// PortBindings is a set of ports to map into the container.
    /// Only available if NetNS is set to bridge, slirp, or pasta.
    /// Optional.
    pub portmappings: Option<Vec<super::super::models::PortMapping>>,
    /// Privileged is whether the container is privileged.
    /// Privileged does the following:
    /// Adds all devices on the system to the container.
    /// Adds all capabilities to the container.
    /// Disables Seccomp, SELinux, and Apparmor confinement.
    /// (Though SELinux can be manually re-enabled).
    /// TODO: this conflicts with things.
    /// TODO: this does more.
    /// Optional.
    pub privileged: Option<bool>,
    /// ProcOpts are the options used for the proc mount.
    pub procfs_opts: Option<Vec<String>>,
    /// PublishExposedPorts will publish ports specified in the image to
    /// random unused ports (guaranteed to be above 1024) on the host.
    /// This is based on ports set in Expose below, and any ports specified
    /// by the Image (if one is given).
    /// Only available if NetNS is set to Bridge or Slirp.
    /// Optional.
    pub publish_image_ports: Option<bool>,
    /// Rlimits are POSIX rlimits to apply to the container.
    /// Optional.
    pub r_limits: Option<Vec<super::super::models::PosixRlimit>>,
    /// RawImageName is the user-specified and unprocessed input referring
    /// to a local or a remote image.
    /// Optional, but strongly encouraged to be set if Image is set.
    pub raw_image_name: Option<String>,
    /// ReadOnlyFilesystem indicates that everything will be mounted
    /// as read-only.
    /// Optional.
    pub read_only_filesystem: Option<bool>,
    /// ReadWriteTmpfs indicates that when running with a ReadOnlyFilesystem
    /// mount temporary file systems.
    /// Optional.
    pub read_write_tmpfs: Option<bool>,
    /// Remove indicates if the container should be removed once it has been started
    /// and exits.
    /// Optional.
    pub remove: Option<bool>,
    pub resource_limits: Option<super::super::models::LinuxResources>,
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
    /// Rootfs is the path to a directory that will be used as the
    /// container's root filesystem. No modification will be made to the
    /// directory, it will be directly mounted into the container as root.
    /// Conflicts with Image.
    /// At least one of Image or Rootfs must be specified.
    pub rootfs: Option<String>,
    /// RootfsMapping specifies if there are UID/GID mappings to apply to the rootfs.
    /// Optional.
    pub rootfs_mapping: Option<String>,
    /// RootfsOverlay tells if rootfs is actually an overlay on top of base path.
    /// Optional.
    pub rootfs_overlay: Option<bool>,
    /// RootfsPropagation is the rootfs propagation mode for the container.
    /// If not set, the default of rslave will be used.
    /// Optional.
    pub rootfs_propagation: Option<String>,
    /// Determine how to handle the NOTIFY_SOCKET - do we participate or pass it through
    /// "container" - let the OCI runtime deal with it, advertise conmon's MAINPID
    /// "conmon-only" - advertise conmon's MAINPID, send READY when started, don't pass to OCI
    /// "ignore" - unset NOTIFY_SOCKET
    /// Optional.
    #[serde(rename = "sdnotifyMode")]
    pub sdnotify_mode: Option<String>,
    /// SeccompPolicy determines which seccomp profile gets applied
    /// the container. valid values: empty,default,image
    pub seccomp_policy: Option<String>,
    /// SeccompProfilePath is the path to a JSON file containing the
    /// container's Seccomp profile.
    /// If not specified, no Seccomp profile will be used.
    /// Optional.
    pub seccomp_profile_path: Option<String>,
    /// EnvSecrets are secrets that will be set as environment variables
    /// Optional.
    pub secret_env: Option<std::collections::HashMap<String, String>>,
    /// Secrets are the secrets that will be added to the container
    /// Optional.
    pub secrets: Option<Vec<super::super::models::Secret>>,
    /// SelinuxProcessLabel is the process label the container will use.
    /// If SELinux is enabled and this is not specified, a label will be
    /// automatically generated if not specified.
    /// Optional.
    pub selinux_opts: Option<Vec<String>>,
    /// ShmSize is the size of the tmpfs to mount in at /dev/shm, in bytes.
    /// Conflicts with ShmSize if IpcNS is not private.
    /// Optional.
    pub shm_size: Option<i64>,
    /// ShmSizeSystemd is the size of systemd-specific tmpfs mounts
    /// specifically /run, /run/lock, /var/log/journal and /tmp.
    /// Optional
    pub shm_size_systemd: Option<i64>,
    #[serde(rename = "startupHealthConfig")]
    pub startup_health_config: Option<super::super::models::StartupHealthCheck>,
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
    /// StorageOpts is the container's storage options
    /// Optional.
    pub storage_opts: Option<std::collections::HashMap<String, String>>,
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
    /// IO read rate limit per cgroup per device, bytes per second
    #[serde(rename = "throttleReadBpsDevice")]
    pub throttle_read_bps_device:
        Option<std::collections::HashMap<String, super::super::models::LinuxThrottleDevice>>,
    /// IO read rate limit per cgroup per device, IO per second
    #[serde(rename = "throttleReadIOPSDevice")]
    pub throttle_read_iops_device:
        Option<std::collections::HashMap<String, super::super::models::LinuxThrottleDevice>>,
    /// IO write rate limit per cgroup per device, bytes per second
    #[serde(rename = "throttleWriteBpsDevice")]
    pub throttle_write_bps_device:
        Option<std::collections::HashMap<String, super::super::models::LinuxThrottleDevice>>,
    /// IO write rate limit per cgroup per device, IO per second
    #[serde(rename = "throttleWriteIOPSDevice")]
    pub throttle_write_iops_device:
        Option<std::collections::HashMap<String, super::super::models::LinuxThrottleDevice>>,
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
    /// Umask is the umask the init process of the container will be run with.
    pub umask: Option<String>,
    /// CgroupConf are key-value options passed into the container runtime
    /// that are used to configure cgroup v2.
    /// Optional.
    pub unified: Option<std::collections::HashMap<String, String>>,
    /// Unmask a path in the container. Some paths are masked by default,
    /// preventing them from being accessed within the container; this undoes
    /// that masking. If ALL is passed, all paths will be unmasked.
    /// Optional.
    pub unmask: Option<Vec<String>>,
    /// UnsetEnv unsets the specified default environment variables from the image or from buildin or containers.conf
    /// Optional.
    pub unsetenv: Option<Vec<String>>,
    /// UnsetEnvAll unsetall default environment variables from the image or from buildin or containers.conf
    /// UnsetEnvAll unsets all default environment variables from the image or from buildin
    /// Optional.
    pub unsetenvall: Option<bool>,
    /// UseImageHosts indicates that /etc/hosts should not be managed by
    /// Podman, and instead sourced from the image.
    /// Conflicts with HostAdd.
    /// Optional.
    pub use_image_hosts: Option<bool>,
    /// UseImageResolvConf indicates that resolv.conf should not be managed
    /// by Podman, but instead sourced from the image.
    /// Conflicts with DNSServer, DNSSearch, DNSOption.
    /// Optional.
    pub use_image_resolve_conf: Option<bool>,
    /// User is the user the container will be run as.
    /// Can be given as a UID or a username; if a username, it will be
    /// resolved within the container, using the container's /etc/passwd.
    /// If unset, the container will be run as root.
    /// Optional.
    pub user: Option<String>,
    pub userns: Option<super::super::models::Namespace>,
    pub utsns: Option<super::super::models::Namespace>,
    /// Volatile specifies whether the container storage can be optimized
    /// at the cost of not syncing all the dirty files in memory.
    /// Optional.
    pub volatile: Option<bool>,
    /// Volumes are named volumes that will be added to the container.
    /// These will supersede Image Volumes and VolumesFrom volumes where
    /// there are conflicts.
    /// Optional.
    pub volumes: Option<Vec<super::super::models::NamedVolume>>,
    /// VolumesFrom is a set of containers whose volumes will be added to
    /// this container. The name or ID of the container must be provided, and
    /// may optionally be followed by a : and then one or more
    /// comma-separated options. Valid options are 'ro', 'rw', and 'z'.
    /// Options will be used for all volumes sourced from the container.
    /// Optional.
    pub volumes_from: Option<Vec<String>>,
    /// Weight per cgroup per device, can override BlkioWeight
    #[serde(rename = "weightDevice")]
    pub weight_device:
        Option<std::collections::HashMap<String, super::super::models::LinuxWeightDevice>>,
    /// WorkDir is the container's working directory.
    /// If unset, the default, /, will be used.
    /// Optional.
    pub work_dir: Option<String>,
}

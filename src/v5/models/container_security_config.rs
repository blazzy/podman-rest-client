use serde::{Deserialize, Serialize};
#[derive(Default, Debug, Serialize, Deserialize)]
/// ContainerSecurityConfig is a container's security features, including
/// SELinux, Apparmor, and Seccomp.
pub struct ContainerSecurityConfig {
    /// ApparmorProfile is the name of the Apparmor profile the container
    /// will use.
    /// Optional.
    pub apparmor_profile: Option<String>,
    /// CapAdd are capabilities which will be added to the container.
    /// Conflicts with Privileged.
    /// Optional.
    pub cap_add: Option<Vec<String>>,
    /// CapDrop are capabilities which will be removed from the container.
    /// Conflicts with Privileged.
    /// Optional.
    pub cap_drop: Option<Vec<String>>,
    /// Groups are a list of supplemental groups the container's user will
    /// be granted access to.
    /// Optional.
    pub groups: Option<Vec<String>>,
    pub idmappings: Option<crate::v5::models::IdMappingOptions>,
    /// LabelNested indicates whether or not the container is allowed to
    /// run fully nested containers including SELinux labelling.
    /// Optional.
    pub label_nested: Option<bool>,
    /// Mask is the path we want to mask in the container. This masks the paths
    /// given in addition to the default list.
    /// Optional
    pub mask: Option<Vec<String>>,
    /// NoNewPrivileges is whether the container will set the no new
    /// privileges flag on create, which disables gaining additional
    /// privileges (e.g. via setuid) in the container.
    /// Optional.
    pub no_new_privileges: Option<bool>,
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
    /// ReadOnlyFilesystem indicates that everything will be mounted
    /// as read-only.
    /// Optional.
    pub read_only_filesystem: Option<bool>,
    /// ReadWriteTmpfs indicates that when running with a ReadOnlyFilesystem
    /// mount temporary file systems.
    /// Optional.
    pub read_write_tmpfs: Option<bool>,
    /// SeccompPolicy determines which seccomp profile gets applied
    /// the container. valid values: empty,default,image
    pub seccomp_policy: Option<String>,
    /// SeccompProfilePath is the path to a JSON file containing the
    /// container's Seccomp profile.
    /// If not specified, no Seccomp profile will be used.
    /// Optional.
    pub seccomp_profile_path: Option<String>,
    /// SelinuxProcessLabel is the process label the container will use.
    /// If SELinux is enabled and this is not specified, a label will be
    /// automatically generated if not specified.
    /// Optional.
    pub selinux_opts: Option<Vec<String>>,
    /// Umask is the umask the init process of the container will be run with.
    pub umask: Option<String>,
    /// Unmask a path in the container. Some paths are masked by default,
    /// preventing them from being accessed within the container; this undoes
    /// that masking. If ALL is passed, all paths will be unmasked.
    /// Optional.
    pub unmask: Option<Vec<String>>,
    /// User is the user the container will be run as.
    /// Can be given as a UID or a username; if a username, it will be
    /// resolved within the container, using the container's /etc/passwd.
    /// If unset, the container will be run as root.
    /// Optional.
    pub user: Option<String>,
    pub userns: Option<crate::v5::models::Namespace>,
}

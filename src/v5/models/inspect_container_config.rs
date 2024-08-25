use serde::{Deserialize, Serialize};
#[derive(Default, Debug, Serialize, Deserialize)]
/// InspectContainerConfig holds further data about how a container was initially
/// configured.
pub struct InspectContainerConfig {
    /// Container annotations
    #[serde(rename = "Annotations")]
    pub annotations: Option<std::collections::HashMap<String, Option<String>>>,
    /// Unused, at present
    #[serde(rename = "AttachStderr")]
    pub attach_stderr: Option<bool>,
    /// Unused, at present
    #[serde(rename = "AttachStdin")]
    pub attach_stdin: Option<bool>,
    /// Unused, at present
    #[serde(rename = "AttachStdout")]
    pub attach_stdout: Option<bool>,
    /// ChrootDirs is an additional set of directories that need to be
    /// treated as root directories. Standard bind mounts will be mounted
    /// into paths relative to these directories.
    #[serde(rename = "ChrootDirs")]
    pub chroot_dirs: Option<Vec<String>>,
    /// Container command
    #[serde(rename = "Cmd")]
    pub cmd: Option<Vec<String>>,
    /// CreateCommand is the full command plus arguments of the process the
    /// container has been created with.
    #[serde(rename = "CreateCommand")]
    pub create_command: Option<Vec<String>>,
    /// Container domain name - unused at present
    #[serde(rename = "Domainname")]
    pub domainname: Option<String>,
    /// Container entrypoint
    #[serde(rename = "Entrypoint")]
    pub entrypoint: Option<Vec<String>>,
    /// Container environment variables
    #[serde(rename = "Env")]
    pub env: Option<Vec<String>>,
    #[serde(rename = "Healthcheck")]
    pub healthcheck: Option<crate::v5::models::Schema2HealthConfig>,
    /// HealthcheckOnFailureAction defines an action to take once the container turns unhealthy.
    #[serde(rename = "HealthcheckOnFailureAction")]
    pub healthcheck_on_failure_action: Option<String>,
    /// Container hostname
    #[serde(rename = "Hostname")]
    pub hostname: Option<String>,
    /// Container image
    #[serde(rename = "Image")]
    pub image: Option<String>,
    /// Container labels
    #[serde(rename = "Labels")]
    pub labels: Option<std::collections::HashMap<String, Option<String>>>,
    /// On-build arguments - presently unused. More of Buildah's domain.
    #[serde(rename = "OnBuild")]
    pub on_build: Option<String>,
    /// Whether the container leaves STDIN open
    #[serde(rename = "OpenStdin")]
    pub open_stdin: Option<bool>,
    /// Passwd determines whether or not podman can add entries to /etc/passwd and /etc/group
    #[serde(rename = "Passwd")]
    pub passwd: Option<bool>,
    /// Secrets are the secrets mounted in the container
    #[serde(rename = "Secrets")]
    pub secrets: Option<Vec<crate::v5::models::InspectSecret>>,
    /// Whether STDIN is only left open once.
    /// Presently not supported by Podman, unused.
    #[serde(rename = "StdinOnce")]
    pub stdin_once: Option<bool>,
    /// Container stop signal
    #[serde(rename = "StopSignal")]
    pub stop_signal: Option<String>,
    /// StopTimeout is time before container is stopped when calling stop
    #[serde(rename = "StopTimeout")]
    pub stop_timeout: Option<u64>,
    /// SystemdMode is whether the container is running in systemd mode. In
    /// systemd mode, the container configuration is customized to optimize
    /// running systemd in the container.
    #[serde(rename = "SystemdMode")]
    pub systemd_mode: Option<bool>,
    /// Timeout is time before container is killed by conmon
    #[serde(rename = "Timeout")]
    pub timeout: Option<u64>,
    /// Timezone is the timezone inside the container.
    /// Local means it has the same timezone as the host machine
    #[serde(rename = "Timezone")]
    pub timezone: Option<String>,
    /// Whether the container creates a TTY
    #[serde(rename = "Tty")]
    pub tty: Option<bool>,
    /// Umask is the umask inside the container.
    #[serde(rename = "Umask")]
    pub umask: Option<String>,
    /// User the container was launched with
    #[serde(rename = "User")]
    pub user: Option<String>,
    /// Unused, at present. I've never seen this field populated.
    #[serde(rename = "Volumes")]
    pub volumes: Option<std::collections::HashMap<String, Option<serde_json::Value>>>,
    /// Container working directory
    #[serde(rename = "WorkingDir")]
    pub working_dir: Option<String>,
    /// SdNotifyMode is the sd-notify mode of the container.
    #[serde(rename = "sdNotifyMode")]
    pub sd_notify_mode: Option<String>,
    /// SdNotifySocket is the NOTIFY_SOCKET in use by/configured for the container.
    #[serde(rename = "sdNotifySocket")]
    pub sd_notify_socket: Option<String>,
}

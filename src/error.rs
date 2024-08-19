#[derive(thiserror::Error, Debug)]
pub enum ClientError {
    #[cfg(feature = "ssh")]
    #[error("SSH error: {0}")]
    Ssh(#[from] russh::Error),
    #[cfg(feature = "ssh")]
    #[error("SSH Key error: {0}")]
    SshKey(#[from] russh_keys::Error),

    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    #[error("Invalid URI: {0}")]
    InvalidUri(#[from] http::uri::InvalidUri),
    #[error("SSH Authentication Failed")]
    AuthenticationFailed,
    #[error("Missing or unsupported scheme in URI")]
    InvalidScheme,
    #[error("Missing SSH user name in URI")]
    SshUserNameRequired,
    #[error("Missing ssh key path")]
    SshKeyPathRequired,
    #[error("Missing SSH host in URI")]
    SshHostRequired,
    #[error("SSH feature flag not enabled. Rebuild with `ssh` to use ssh uris")]
    SshFeatureFlagNotEnabled,
    #[error("Unix domain socket feature flag not enabled. Rebuild with `uds` to use unix uris")]
    UdsFeatureFlagNotEnabled,
}

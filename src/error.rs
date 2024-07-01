#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("SSH error: {0}")]
    Ssh(#[from] russh::Error),
    #[error("SSH Key error: {0}")]
    SshKey(#[from] russh_keys::Error),
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    #[error("Invalid URI: {0}")]
    InvalidUri(#[from] http::uri::InvalidUri),
    #[error("SSH Authentication Failed")]
    AuthenticationFailed,

    #[error("Missing or unkown scheme in URI")]
    InvalidScheme,
    #[error("Missing SSH user name in URI")]
    SshUserNameRequired,
    #[error("Missing ssh key path")]
    SshKeyPathRequired,
    #[error("Missing SSH host in URI")]
    SshHostRequired,
}

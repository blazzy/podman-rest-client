#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("SSH error: {0}")]
    Ssh(#[from] russh::Error),
    #[error("SSH Key error: {0}")]
    SshKey(#[from] russh_keys::Error),
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    #[error("SSH Authentication Failed")]
    AuthenticationFailed,
}

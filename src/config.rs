use crate::cli;

pub struct Config {
    pub uri: String,
    pub identity_file: Option<String>,
}

#[derive(thiserror::Error, Debug)]
pub enum GuessError {
    #[error("No default podman connection info found")]
    NoDefault,
    #[error("Cli error: {0}")]
    Cli(#[from] cli::CliError),
}

impl Config {
    pub async fn guess() -> Result<Config, GuessError> {
        if cfg!(target_os = "macos") {
            let default = cli::get_default_system_connection()
                .await?
                .ok_or(GuessError::NoDefault)?;

            Ok(Config {
                uri: default.uri,
                identity_file: default.identity,
            })
        } else {
            let uid = nix::unistd::getuid();
            let user_socket_path = format!("unix:///run/user/{}/podman/podman.sock", uid);

            let user_socket_exists = std::path::Path::new(&user_socket_path).exists();

            if user_socket_exists {
                return Ok(Config {
                    uri: user_socket_path,
                    identity_file: None,
                });
            }

            let system_socket_path = "/run/podman/podman.sock";
            let system_socket_exists = std::path::Path::new(&system_socket_path).exists();
            if system_socket_exists {
                return Ok(Config {
                    uri: system_socket_path.to_string(),
                    identity_file: None,
                });
            }

            Err(GuessError::NoDefault)
        }
    }
}

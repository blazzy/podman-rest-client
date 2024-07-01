//! Provides an interface for querying the podman rest api. Most of the interface is generated from
//! the the official podman swagger file. This crate adds a layer to make it possible to connect to
//! the podman rest api over ssh to a unix socket and directl to a unix socket. Connections over
//! ssh are  commonly necessary on macOs where the container runtime runs in a virtual machine
//! accessible over ssh.
//!
//! ```no_run
//! # tokio_test::block_on(async {
//! use podman_rest_client::PodmanRestClient;
//! use podman_rest_client::guess_configuration;
//! ;
//!
//! let config = guess_configuration().await.unwrap();
//!
//! let client = PodmanRestClient::new(config).await.unwrap();
//!
//! let images = client.images_api().image_list_libpod(None,None).await.unwrap();
//! # })
//! ```
//!
//! If guess_configuration doesn't work for you you can manually create a config.
//!
//! ```no_run
//! # tokio_test::block_on(async {
//! use podman_rest_client::PodmanRestClient;
//! use podman_rest_client::Config;
//!
//! let ssh_client = PodmanRestClient::new(Config {
//!     uri: "ssh://core@127.0.0.1:63169/run/user/501/podman/podman.sock".to_string(),
//!     identity_file: Some("/path/to/identity_file".into()),
//! }).await.unwrap();
//!
//! let unix_client = PodmanRestClient::new(Config {
//!     uri: "unix://run/user/501/podman/podman.sock".to_string(),
//!     identity_file: None,
//! }).await.unwrap();
//! # })
//! ```

pub mod cli;
mod error;
mod podman_rest_client;
mod ssh;
mod unix_socket;

pub use error::Error;
pub use podman_rest_client::Config;
pub use podman_rest_client::PodmanRestClient;

#[derive(thiserror::Error, Debug)]
pub enum GuessError {
    #[error("No default podman connection info found")]
    NoDefault,
    #[error("Cli error: {0}")]
    Cli(#[from] cli::CliError),
}

#[cfg(target_os = "macos")]
pub async fn guess_configuration() -> Result<Config, GuessError> {
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
        Ok(Config {
            uri: format!("/run/user/{}/podman/podman.sock", uid),
            identity_file: None,
        })
    }
}

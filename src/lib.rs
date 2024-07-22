//! Provides an interface for querying the Podman REST API. Most of the interface is generated from
//! the the official Podman swagger file. This crate adds a layer to make it possible to connect to
//! the podman rest api over ssh to a unix socket and directly to a unix socket. Connections over
//! ssh are  commonly necessary on macOs where the container runtime runs in a virtual machine
//! accessible over ssh.
//!
//!
//! ### API Compatibility
//!
//! This crate currently only works with version 5 of the podman API. There are suffucient
//! differences between version 3, 4, and 5 that a lot of calls will not work in an older version.
//! `podman --version` will reveal what version you are using.
//!
//! ## Usage
//!
//! ```no_run
//! # tokio_test::block_on(async {
//! use podman_rest_client::PodmanRestClient;
//! use podman_rest_client::guess_configuration;
//!
//! // Setup the default configuration
//! let config = guess_configuration().await.unwrap();
//!
//! // Initialize a client
//! let client = PodmanRestClient::new(config).await.unwrap();
//!
//! // Fetch a list of container images
//! let images = client.images_api().image_list_libpod(None,None).await.unwrap();
//! # })
//! ```
//!
//! `guess_configuration` tries to find the default path to the podman socket depending on the
//! platform you are on. You can also manually create clients configurations:
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
//!     uri: "unix:///run/user/501/podman/podman.sock".to_string(),
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
pub use podman_autogen_api::apis;
pub use podman_autogen_api::models;
pub use podman_rest_client::Config;
pub use podman_rest_client::PodmanRestClient;

#[derive(thiserror::Error, Debug)]
pub enum GuessError {
    #[error("No default podman connection info found")]
    NoDefault,
    #[error("Cli error: {0}")]
    Cli(#[from] cli::CliError),
}

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
            uri: format!("unix:///run/user/{}/podman/podman.sock", uid),
            identity_file: None,
        })
    }
}

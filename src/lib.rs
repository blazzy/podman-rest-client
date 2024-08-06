//! Provides an interface for querying the Podman REST API. Most of the interface is generated from
//! the the official Podman swagger file. This crate adds a layer to make it possible to connect to
//! the podman rest api over ssh to a unix socket and directly to a unix socket. Connections over
//! ssh are  commonly necessary on macOs where the container runtime runs in a virtual machine
//! accessible over ssh.
//!
//!
//! ## API Compatibility
//!
//! This crate currently only works with version 5 of the podman API. There are suffucient
//! differences between version 3, 4, and 5 that a lot of calls will not work in an older version.
//! `podman --version` will reveal what version you are using.
//!
//! ## Podman Socket
//!
//! Note that podman does not run in a client/server model like docker does so there usually isn't
//! a socket you can connect to by default. You might need to enable the socket for the library to
//! connect to. For example on linux you might need to run something like this:
//!
//! ```sh
//! systemctl --user enable --now podman.socket
//! ```
//!
//! ## Usage
//!
//! ### Linux
//!
//! On linux you might initialize a client like this
//!
//! ```no_run
//! # tokio_test::block_on(async {
//! use podman_rest_client::PodmanRestClient;
//! use podman_rest_client::Config;
//!
//! // Initialize a client
//! let client = PodmanRestClient::new(Config {
//!     uri: "unix:///run/user/501/podman/podman.sock".to_string(),
//!     identity_file: None,
//! }).await.unwrap();
//!
//! // Fetch a list of container images
//! let images = client.images().image_list_libpod(None).await.unwrap();
//! # })
//! ```
//! ### MacOs
//!
//! On macOs you might initialize a client like this with an ssh url and identity file
//!
//! ```no_run
//! # tokio_test::block_on(async {
//! # use podman_rest_client::PodmanRestClient;
//! # use podman_rest_client::Config;
//! let client = PodmanRestClient::new(Config {
//!     uri: "ssh://core@127.0.0.1:63169/run/user/501/podman/podman.sock".to_string(),
//!     identity_file: Some("/path/to/identity_file".into()),
//! }).await.unwrap();
//! # })
//! ```
//!
//! ### Config::guess
//!
//! You can also use `Config::guess()` which tries to find the default path to the podman
//! socket depending on the platform you are on.
//!
//! ```no_run
//! # tokio_test::block_on(async {
//! # use podman_rest_client::PodmanRestClient;
//! # use podman_rest_client::Config;
//! // Setup the default configuration
//! let config = Config::guess().await.unwrap();
//!
//! // Initialize a client
//! let client = PodmanRestClient::new(config).await.unwrap();
//!
//! // Fetch a list of container images
//! let images = client.images().image_list_libpod(None).await.unwrap();
//! # })
//! ```

pub mod cli;
mod config;
mod error;
mod podman_rest_client;
mod ssh;
mod unix_socket;

pub use config::Config;
pub use error::Error;
pub use podman_autogen_api::apis;
pub use podman_autogen_api::models;
pub use podman_autogen_api::params;
pub use podman_rest_client::PodmanRestClient;

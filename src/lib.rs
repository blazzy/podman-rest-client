#![feature(doc_cfg)]
//! Provides an interface for querying the Podman REST API. Most of the interface is generated from
//! the official Podman swagger file. It can connect to the Podman API over ssh to a unix socket
//! and directly to a unix socket. Connections over ssh are  commonly necessary on macOs where the
//! container runtime runs in a virtual machine accessible over ssh.
//!
//!
//! ## API Compatibility
//!
//! Use `podman --version` to determine what version of Podman you are using.
//!
//! ### v5 Support
//!
//! This crate primarily works with version 5 of the Podman API. There are sufficient differences
//! between version 3, 4, and 5 that a lot of calls will not work in an older version.
//!
//! ### v4 Support (Not in good shape)
//!
//! While there is tentative v4 support it's in pretty terrible shape because the official Podman
//! swagger file is missing all kinds of definitions. Some have been manually created, there is a
//! lot more to do.
//!
//! ## Podman Socket
//!
//! Note that podman does not run in a client/server model like docker does so there usually isn't
//! a socket you can connect to by default. You might need to enable the socket for the library to
//! connect to. For example on linux you might need to run something like this:
//!
//! ```sh
//! systemctl --user enable --now podman.socket // Enable the podman unix domain socket
//! ```
//!
//! On macOS you might need to invoke something like:
//!
//! ```sh
//! podman machine init // Create your podman virtual machine
//! podman machine start // Start the machine
//! ```
//!
//! ## Usage
//!
//! ### Linux
//!
//! On linux you might initialize a client like this
//!
//! ```no_run
//! # #[cfg(feature = "v5")]
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
//! let images = client.v5().images().image_list_libpod(None).await.unwrap();
//! # })
//! ```
//! ### MacOs
//!
//! On macOs you might initialize a client like this with an ssh url and identity file
//!
//! ```no_run
//! # #[cfg(feature = "v5")]
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
//! # #[cfg(feature = "v5")]
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
//! let images = client.v5().images().image_list_libpod(None).await.unwrap();
//! # })
//! ```
//!
//! ### Client/API Traits
//!
//! If you import the `podman_rest_client::v5::Client` trait you  can directly call the api
//! functions from a client:
//!
//! ```
//! # #[cfg(feature = "v5")]
//! # tokio_test::block_on(async {
//! # use podman_rest_client::PodmanRestClient;
//! # use podman_rest_client::Config;
//! use podman_rest_client::v5::Client;
//! # let config = Config::guess().await.unwrap();
//! # // Initialize a client
//! # let client = PodmanRestClient::new(config).await.unwrap();
//! client.images().image_list_libpod(None).await;
//! # })
//! ```
//!
//! You can also use various api traits like `podman_rest_client::v5::apis::Images` and directly
//! call the individual request functions:
//!
//! ```
//! # #[cfg(feature = "v5")]
//! # tokio_test::block_on(async {
//! # use podman_rest_client::PodmanRestClient;
//! # use podman_rest_client::Config;
//! use podman_rest_client::v5::apis::Images;
//! # let config = Config::guess().await.unwrap();
//! # // Initialize a client
//! # let client = PodmanRestClient::new(config).await.unwrap();
//! client.image_list_libpod(None).await;
//! # })
//! ```
//!
//!
//! ## Features
//!
//! The default feature set is `["v5", "uds", "ssh"]`.
//!
//! - `ssh`: Support for connecting to a podman through an ssh server.
//! - `uds`: Support for connecting to podman through a unix domain socket.
//! - `v5`: Support for version 5 of the podman API
//! - `v4`: Support for version 4 of the podman API. v4 is nowhere near ready for use.
//!

pub mod cli;
mod config;
mod error;
mod podman_rest_client;
#[cfg(feature = "ssh")]
mod ssh;
#[cfg(feature = "uds")]
mod unix_socket;

#[cfg(feature = "v4")]
pub mod v4;

pub mod api_common;

#[cfg(feature = "v5")]
pub mod v5;
pub use config::Config;
pub use error::Error;
pub use podman_rest_client::PodmanRestClient;

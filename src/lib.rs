//! Provides an interface for querying the podman rest api. Most of the interface is generated from
//! the the official podman swagger file. This crate adds a layer to make it possible to connect to
//! podman over ssh. This is commonly necessary on macOs where the container runtime runs in a
//! virtual machine you connect to over ssh.
//!
//! ```
//! let client = PodmanRestCli
//! ```
//!

pub mod cli;
mod error;
mod podman_rest_client;
mod ssh;

pub use error::Error;
pub use podman_rest_client::Config;
pub use podman_rest_client::PodmanRestClient;

/*
impl PodmanConnection {
    fn into_config(&mut self) -> Result<Config, hyper::http::uri::InvalidUri> {
        let uri = hyper::Uri::from_str(&self.uri)?;
        let user_name = uri.authority().and_then(|authority| {
            if let Some((user_name, _)) = authority.to_string().split_once('@') {
                Some(user_name.to_string())
            } else {
                None
            }
        });

        Ok(())
    }
}*/

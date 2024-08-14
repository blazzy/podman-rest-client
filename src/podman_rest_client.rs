use std::str::FromStr;

use hyper_util::client::legacy::Client;
use hyper_util::rt::TokioExecutor;

use crate::config::Config;
use crate::error::Error;
#[cfg(feature = "v4")]
use crate::impl_crate_v4_traits;
#[cfg(feature = "v5")]
use crate::impl_crate_v5_traits;
use crate::ssh;
use crate::unix_socket;
use crate::APIConfig;
use crate::ClientConfig;
use crate::Connector;
use crate::HasConfig;

const BASE_PATH: &str = "http://d/v5.1.0";

pub struct PodmanRestClient {
    config: Box<dyn ClientConfig>,
}

#[cfg(feature = "v5")]
impl_crate_v5_traits!(PodmanRestClient);

impl HasConfig for PodmanRestClient {
    fn get_config(&self) -> &dyn ClientConfig {
        &*self.config
    }
}

#[cfg(feature = "v4")]
impl_crate_v4_traits!(PodmanRestClient);

impl PodmanRestClient {
    pub async fn new(config: Config) -> Result<Self, Error> {
        let (scheme, rest) = config.uri.split_once("://").ok_or(Error::InvalidScheme)?;

        match scheme {
            "unix" => Ok(PodmanRestClient::new_unix(rest)),
            "ssh" => PodmanRestClient::new_ssh(config.uri, config.identity_file).await,
            _ => Err(Error::InvalidScheme),
        }
    }

    pub async fn new_ssh(uri: String, key_path: Option<String>) -> Result<PodmanRestClient, Error> {
        let uri = hyper::Uri::from_str(&uri)?;

        let user_name = uri.authority().and_then(|authority| {
            if let Some((user_name, _)) = authority.to_string().split_once('@') {
                Some(user_name.to_string())
            } else {
                None
            }
        });

        let user_name = user_name.ok_or(Error::SshUserNameRequired)?;
        let key_path = key_path.ok_or(Error::SshKeyPathRequired)?;
        let host = uri.host().ok_or(Error::SshHostRequired)?;
        let address = uri
            .port()
            .map_or(host.to_string(), |port| format!("{}:{}", host, port));

        let connector = ssh::SshConnector::new(&user_name, &key_path, &address, uri.path()).await?;

        Ok(PodmanRestClient::new_connector(connector))
    }

    pub fn new_unix(path: &str) -> PodmanRestClient {
        let connector = unix_socket::UnixConnector::new(path);

        PodmanRestClient::new_connector(connector)
    }

    fn new_connector<C: Connector>(connector: C) -> PodmanRestClient {
        let client = Client::builder(TokioExecutor::new()).build(connector);

        PodmanRestClient {
            config: Box::new(APIConfig {
                base_path: BASE_PATH.to_string(),
                ..APIConfig::with_client(client)
            }),
        }
    }
}

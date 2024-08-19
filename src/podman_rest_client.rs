use std::str::FromStr;

use hyper_util::client::legacy::Client;
use hyper_util::rt::TokioExecutor;

use crate::api_common::config::HasConfig;
use crate::api_common::ClientConfig;
use crate::api_common::Config as APIConfig;
use crate::api_common::Connector;
use crate::config::Config;
use crate::error::ClientError;
#[cfg(feature = "v4")]
use crate::impl_crate_v4_traits;
#[cfg(feature = "v5")]
use crate::impl_crate_v5_traits;
#[cfg(feature = "ssh")]
use crate::ssh;
#[cfg(feature = "uds")]
use crate::unix_socket;

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
    pub async fn new(config: Config) -> Result<Self, ClientError> {
        let (scheme, rest) = config
            .uri
            .split_once("://")
            .ok_or(ClientError::InvalidScheme)?;

        match scheme {
            #[cfg(feature = "uds")]
            "unix" => Ok(PodmanRestClient::new_unix(rest)),
            #[cfg(not(feature = "uds"))]
            "uds" => Err(ClientError::UdsFeatureFlagNotEnabled),
            #[cfg(feature = "ssh")]
            "ssh" => PodmanRestClient::new_ssh(config.uri, config.identity_file).await,
            #[cfg(not(feature = "ssh"))]
            "ssh" => Err(ClientError::SshFeatureFlagNotEnabled),
            _ => Err(ClientError::InvalidScheme),
        }
    }

    #[cfg_attr(docsrs, doc(cfg(feature = "ssh")))]
    #[cfg(feature = "ssh")]
    pub async fn new_ssh(
        uri: String,
        key_path: Option<String>,
    ) -> Result<PodmanRestClient, ClientError> {
        let uri = hyper::Uri::from_str(&uri)?;

        let user_name = uri.authority().and_then(|authority| {
            if let Some((user_name, _)) = authority.to_string().split_once('@') {
                Some(user_name.to_string())
            } else {
                None
            }
        });

        let user_name = user_name.ok_or(ClientError::SshUserNameRequired)?;
        let key_path = key_path.ok_or(ClientError::SshKeyPathRequired)?;
        let host = uri.host().ok_or(ClientError::SshHostRequired)?;
        let address = uri
            .port()
            .map_or(host.to_string(), |port| format!("{}:{}", host, port));

        let connector = ssh::SshConnector::new(&user_name, &key_path, &address, uri.path()).await?;

        Ok(PodmanRestClient::new_connector(connector))
    }

    #[cfg_attr(docsrs, doc(cfg(feature = "uds")))]
    #[cfg(feature = "uds")]
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

    #[cfg(feature = "v5")]
    pub fn v5(&self) -> &dyn crate::v5::Client {
        self
    }

    #[cfg_attr(docsrs, doc(cfg(feature = "v4")))]
    #[cfg(feature = "v4")]
    pub fn v4(&self) -> &dyn crate::v4::Client {
        self
    }
}

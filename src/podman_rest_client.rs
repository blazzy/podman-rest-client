use std::ops::Deref;
use std::str::FromStr;

use hyper_util::client::legacy::connect::Connect;
use hyper_util::client::legacy::Client;
use hyper_util::rt::TokioExecutor;
use podman_autogen_api::apis::client::APIClient;
use podman_autogen_api::apis::configuration::Configuration as APIConfiguration;

use crate::config::Config;
use crate::error::Error;
use crate::ssh;
use crate::unix_socket;

const BASE_PATH: &str = "http://d/v5.1.0";

pub struct PodmanRestClient(pub APIClient);

impl PodmanRestClient {
    pub async fn new(config: Config) -> Result<PodmanRestClient, Error> {
        let (scheme, rest) = config.uri.split_once("://").ok_or(Error::InvalidScheme)?;

        match scheme {
            "unix" => PodmanRestClient::new_unix(rest).await,
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

        PodmanRestClient::new_connector(connector).await
    }

    pub async fn new_unix(path: &str) -> Result<PodmanRestClient, Error> {
        let connector = unix_socket::UnixConnector::new(path);

        PodmanRestClient::new_connector(connector).await
    }

    async fn new_connector<C>(connector: C) -> Result<PodmanRestClient, Error>
    where
        C: Connect + Clone + Send + Sync + 'static,
    {
        let client = Client::builder(TokioExecutor::new()).build(connector);

        let configuration = APIConfiguration {
            base_path: BASE_PATH.to_string(),
            ..APIConfiguration::with_client(client)
        };
        let api_client = APIClient::new(configuration);

        Ok(PodmanRestClient(api_client))
    }
}

impl Deref for PodmanRestClient {
    type Target = APIClient;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

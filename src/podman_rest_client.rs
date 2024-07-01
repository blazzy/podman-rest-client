use std::ops::Deref;
use std::str::FromStr;

use hyper_util::client::legacy::connect::Connect;
use hyper_util::client::legacy::Client;
use hyper_util::rt::TokioExecutor;
use podman_autogen_api::apis::client::APIClient;
use podman_autogen_api::apis::configuration::Configuration as APIConfiguration;

use crate::error::Error;
use crate::ssh;
use crate::unix_socket;

const BASE_PATH: &str = "http://d/v5.1.0";

pub struct PodmanRestClient(pub APIClient);

pub struct Config {
    pub uri: String,
    pub identity_file: Option<String>,
}

impl PodmanRestClient {
    pub async fn new(config: Config) -> Result<PodmanRestClient, Error> {
        let uri = hyper::Uri::from_str(&config.uri)?;

        if let Some(scheme) = uri.scheme() {
            match scheme.as_str() {
                "unix" => PodmanRestClient::new_unix(uri).await,
                "ssh" => PodmanRestClient::new_ssh(uri, config.identity_file).await,
                _ => Err(Error::InvalidScheme),
            }
        } else {
            Err(Error::InvalidScheme)
        }
    }

    pub async fn new_ssh(
        uri: hyper::Uri,
        key_path: Option<String>,
    ) -> Result<PodmanRestClient, Error> {
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

    pub async fn new_unix(uri: hyper::Uri) -> Result<PodmanRestClient, Error> {
        let connector = unix_socket::UnixConnector::new(uri.to_string());

        PodmanRestClient::new_connector(connector).await
    }

    async fn new_connector<C>(connector: C) -> Result<PodmanRestClient, Error>
    where
        C: Connect + Clone + Send + Sync + 'static,
    {
        let client = Client::builder(TokioExecutor::new()).build(connector);

        let configuration = APIConfiguration {
            base_path: BASE_PATH.to_string(),
            ..APIConfiguration::new(client)
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

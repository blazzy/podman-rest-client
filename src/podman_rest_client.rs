use std::ops::Deref;

use hyper_util::rt::TokioExecutor;
use podman_api::apis::client::APIClient;
use podman_api::apis::configuration::Configuration as APIConfiguration;

use crate::ssh;
use crate::error;

pub struct PodmanRestClient(pub APIClient);

struct SshConfiguration {
    pub user: String,
    key: String,
    address: String,
    socket_path: String,
}

pub struct Config {
    socket: Option<String>,
    ssh: Option<SshConfiguration>,
}

impl PodmanRestClient {
    pub async fn new(config: Config) -> Result<PodmanRestClient, error::Error> {
        let ssh_config = config.ssh.unwrap();

        let connector = ssh::SshConnector::new(
            &ssh_config.user,
            &ssh_config.key,
            &ssh_config.address,
            &ssh_config.socket_path,
        )
        .await?;

        let client =
            hyper_util::client::legacy::Client::builder(TokioExecutor::new()).build(connector);

        let configuration = APIConfiguration {
            base_path: "http://d/v5.1.0".to_string(),
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

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn it_works() {
        let client = PodmanRestClient::new(Config {
            socket: None,
            ssh: Some(SshConfiguration {
                user: "core@127.0.0.1:63169".into(),
                key: "/Users/blazzy/.local/share/containers/podman/machine/machine".into(),
                address: "127.0.0.1:63169".into(),
                socket_path: "/run/user/501/podman/podman.sock".into(),
            }),
        })
        .await
        .unwrap();
        let images = client
            .images_api()
            .image_list_libpod(None, None)
            .await
            .unwrap();
        println!("{:?}", images);

        assert_eq!(4, 4);
    }
}

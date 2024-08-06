use std::sync::Arc;

use super::apis;
use super::config::ClientConfig;
use super::config::Config;
use super::config::Connector;

pub struct Client {
    config: Arc<dyn ClientConfig>,
}

impl Client {
    pub fn new<C: Connector>(config: Config<C>) -> Client {
        Client {
            config: Arc::new(config),
        }
    }

    /// Actions related to containers
    pub fn containers(&self) -> apis::Containers {
        apis::Containers::new(self.config.clone())
    }

    /// Actions related to containers for the compatibility endpoints
    pub fn containers_compat(&self) -> apis::ContainersCompat {
        apis::ContainersCompat::new(self.config.clone())
    }

    /// Actions related to exec
    pub fn exec(&self) -> apis::Exec {
        apis::Exec::new(self.config.clone())
    }

    /// Actions related to exec for the compatibility endpoints
    pub fn exec_compat(&self) -> apis::ExecCompat {
        apis::ExecCompat::new(self.config.clone())
    }

    /// Actions related to images
    pub fn images(&self) -> apis::Images {
        apis::Images::new(self.config.clone())
    }

    /// Actions related to images for the compatibility endpoints
    pub fn images_compat(&self) -> apis::ImagesCompat {
        apis::ImagesCompat::new(self.config.clone())
    }

    /// Actions related to manifests
    pub fn manifests(&self) -> apis::Manifests {
        apis::Manifests::new(self.config.clone())
    }

    /// Actions related to networks
    pub fn networks(&self) -> apis::Networks {
        apis::Networks::new(self.config.clone())
    }

    /// Actions related to networks for the compatibility endpoints
    pub fn networks_compat(&self) -> apis::NetworksCompat {
        apis::NetworksCompat::new(self.config.clone())
    }

    /// Actions related to pods
    pub fn pods(&self) -> apis::Pods {
        apis::Pods::new(self.config.clone())
    }

    /// Actions related to secrets
    pub fn secrets(&self) -> apis::Secrets {
        apis::Secrets::new(self.config.clone())
    }

    /// Actions related to secrets for the compatibility endpoints
    pub fn secrets_compat(&self) -> apis::SecretsCompat {
        apis::SecretsCompat::new(self.config.clone())
    }

    /// Actions related to Podman engine
    pub fn system(&self) -> apis::System {
        apis::System::new(self.config.clone())
    }

    /// Actions related to Podman and compatibility engines
    pub fn system_compat(&self) -> apis::SystemCompat {
        apis::SystemCompat::new(self.config.clone())
    }

    /// Actions related to volumes
    pub fn volumes(&self) -> apis::Volumes {
        apis::Volumes::new(self.config.clone())
    }

    /// Actions related to volumes for the compatibility endpoints
    pub fn volumes_compat(&self) -> apis::VolumesCompat {
        apis::VolumesCompat::new(self.config.clone())
    }
}

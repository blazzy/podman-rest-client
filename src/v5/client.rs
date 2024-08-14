use super::apis;
use crate::api_common::config::ClientConfig;
use crate::api_common::config::Config;
use crate::api_common::config::Connector;
use crate::api_common::config::HasConfig;
use crate::impl_crate_v5_traits;
use std::marker::Sized;
pub struct APIClient {
    config: Box<dyn ClientConfig>,
}
impl APIClient {
    pub fn new<C: Connector>(config: Config<C>) -> APIClient {
        APIClient {
            config: Box::new(config),
        }
    }
}
impl HasConfig for APIClient {
    fn get_config(&self) -> &dyn ClientConfig {
        &*self.config
    }
}
pub trait Client:
    HasConfig
    + Send
    + Sync
    + Sized
    + apis::Containers
    + apis::ContainersCompat
    + apis::Exec
    + apis::ExecCompat
    + apis::Images
    + apis::ImagesCompat
    + apis::Manifests
    + apis::Networks
    + apis::NetworksCompat
    + apis::Pods
    + apis::Secrets
    + apis::SecretsCompat
    + apis::System
    + apis::SystemCompat
    + apis::Volumes
    + apis::VolumesCompat
{
    /// Actions related to containers
    fn containers(&self) -> &dyn apis::Containers {
        self
    }
    /// Actions related to containers for the compatibility endpoints
    fn containers_compat(&self) -> &dyn apis::ContainersCompat {
        self
    }
    /// Actions related to exec
    fn exec(&self) -> &dyn apis::Exec {
        self
    }
    /// Actions related to exec for the compatibility endpoints
    fn exec_compat(&self) -> &dyn apis::ExecCompat {
        self
    }
    /// Actions related to images
    fn images(&self) -> &dyn apis::Images {
        self
    }
    /// Actions related to images for the compatibility endpoints
    fn images_compat(&self) -> &dyn apis::ImagesCompat {
        self
    }
    /// Actions related to manifests
    fn manifests(&self) -> &dyn apis::Manifests {
        self
    }
    /// Actions related to networks
    fn networks(&self) -> &dyn apis::Networks {
        self
    }
    /// Actions related to networks for the compatibility endpoints
    fn networks_compat(&self) -> &dyn apis::NetworksCompat {
        self
    }
    /// Actions related to pods
    fn pods(&self) -> &dyn apis::Pods {
        self
    }
    /// Actions related to secrets
    fn secrets(&self) -> &dyn apis::Secrets {
        self
    }
    /// Actions related to secrets for the compatibility endpoints
    fn secrets_compat(&self) -> &dyn apis::SecretsCompat {
        self
    }
    /// Actions related to Podman engine
    fn system(&self) -> &dyn apis::System {
        self
    }
    /// Actions related to Podman and compatibility engines
    fn system_compat(&self) -> &dyn apis::SystemCompat {
        self
    }
    /// Actions related to volumes
    fn volumes(&self) -> &dyn apis::Volumes {
        self
    }
    /// Actions related to volumes for the compatibility endpoints
    fn volumes_compat(&self) -> &dyn apis::VolumesCompat {
        self
    }
}
impl_crate_v5_traits!(APIClient);
#[macro_export]
macro_rules! impl_crate_v5_traits {
    ($struct_name:ident) => {
        impl crate::v5::Client for $struct_name {}
        impl crate::v5::apis::Containers for $struct_name {}
        impl crate::v5::apis::ContainersCompat for $struct_name {}
        impl crate::v5::apis::Exec for $struct_name {}
        impl crate::v5::apis::ExecCompat for $struct_name {}
        impl crate::v5::apis::Images for $struct_name {}
        impl crate::v5::apis::ImagesCompat for $struct_name {}
        impl crate::v5::apis::Manifests for $struct_name {}
        impl crate::v5::apis::Networks for $struct_name {}
        impl crate::v5::apis::NetworksCompat for $struct_name {}
        impl crate::v5::apis::Pods for $struct_name {}
        impl crate::v5::apis::Secrets for $struct_name {}
        impl crate::v5::apis::SecretsCompat for $struct_name {}
        impl crate::v5::apis::System for $struct_name {}
        impl crate::v5::apis::SystemCompat for $struct_name {}
        impl crate::v5::apis::Volumes for $struct_name {}
        impl crate::v5::apis::VolumesCompat for $struct_name {}
    };
}

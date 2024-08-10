use super::apis;
use super::config::ClientConfig;
use super::config::Config;
use super::config::Connector;
use super::config::HasConfig;
use crate::impl_api_client;
pub struct Client {
    config: Box<dyn ClientConfig>,
}
impl Client {
    pub fn new<C: Connector>(config: Config<C>) -> Client {
        Client {
            config: Box::new(config),
        }
    }
}
impl_api_client!(Client, config);
#[macro_export]
macro_rules! impl_api_client {
    ($struct_name:ident, $config_field:ident) => {
        impl HasConfig for $struct_name {
            fn get_config(&self) -> &dyn ClientConfig {
                &*self.$config_field
            }
        }
        impl $struct_name {
            #[doc = " Actions related to containers"]
            pub fn containers(&self) -> &dyn apis::Containers {
                self
            }
            #[doc = " Actions related to containers for the compatibility endpoints"]
            pub fn containers_compat(&self) -> &dyn apis::ContainersCompat {
                self
            }
            #[doc = " Actions related to exec"]
            pub fn exec(&self) -> &dyn apis::Exec {
                self
            }
            #[doc = " Actions related to exec for the compatibility endpoints"]
            pub fn exec_compat(&self) -> &dyn apis::ExecCompat {
                self
            }
            #[doc = " Actions related to images"]
            pub fn images(&self) -> &dyn apis::Images {
                self
            }
            #[doc = " Actions related to images for the compatibility endpoints"]
            pub fn images_compat(&self) -> &dyn apis::ImagesCompat {
                self
            }
            #[doc = " Actions related to manifests"]
            pub fn manifests(&self) -> &dyn apis::Manifests {
                self
            }
            #[doc = " Actions related to networks"]
            pub fn networks(&self) -> &dyn apis::Networks {
                self
            }
            #[doc = " Actions related to networks for the compatibility endpoints"]
            pub fn networks_compat(&self) -> &dyn apis::NetworksCompat {
                self
            }
            #[doc = " Actions related to pods"]
            pub fn pods(&self) -> &dyn apis::Pods {
                self
            }
            #[doc = " Actions related to secrets"]
            pub fn secrets(&self) -> &dyn apis::Secrets {
                self
            }
            #[doc = " Actions related to secrets for the compatibility endpoints"]
            pub fn secrets_compat(&self) -> &dyn apis::SecretsCompat {
                self
            }
            #[doc = " Actions related to Podman engine"]
            pub fn system(&self) -> &dyn apis::System {
                self
            }
            #[doc = " Actions related to Podman and compatibility engines"]
            pub fn system_compat(&self) -> &dyn apis::SystemCompat {
                self
            }
            #[doc = " Actions related to volumes"]
            pub fn volumes(&self) -> &dyn apis::Volumes {
                self
            }
            #[doc = " Actions related to volumes for the compatibility endpoints"]
            pub fn volumes_compat(&self) -> &dyn apis::VolumesCompat {
                self
            }
        }
        impl apis::Containers for $struct_name {}
        impl apis::ContainersCompat for $struct_name {}
        impl apis::Exec for $struct_name {}
        impl apis::ExecCompat for $struct_name {}
        impl apis::Images for $struct_name {}
        impl apis::ImagesCompat for $struct_name {}
        impl apis::Manifests for $struct_name {}
        impl apis::Networks for $struct_name {}
        impl apis::NetworksCompat for $struct_name {}
        impl apis::Pods for $struct_name {}
        impl apis::Secrets for $struct_name {}
        impl apis::SecretsCompat for $struct_name {}
        impl apis::System for $struct_name {}
        impl apis::SystemCompat for $struct_name {}
        impl apis::Volumes for $struct_name {}
        impl apis::VolumesCompat for $struct_name {}
    };
}

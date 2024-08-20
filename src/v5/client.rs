use super::apis;
use crate::api_common::config::HasConfig;
pub trait Client:
    HasConfig
    + Send
    + Sync
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
    fn containers(&self) -> &dyn apis::Containers;
    /// Actions related to containers for the compatibility endpoints
    fn containers_compat(&self) -> &dyn apis::ContainersCompat;
    /// Actions related to exec
    fn exec(&self) -> &dyn apis::Exec;
    /// Actions related to exec for the compatibility endpoints
    fn exec_compat(&self) -> &dyn apis::ExecCompat;
    /// Actions related to images
    fn images(&self) -> &dyn apis::Images;
    /// Actions related to images for the compatibility endpoints
    fn images_compat(&self) -> &dyn apis::ImagesCompat;
    /// Actions related to manifests
    fn manifests(&self) -> &dyn apis::Manifests;
    /// Actions related to networks
    fn networks(&self) -> &dyn apis::Networks;
    /// Actions related to networks for the compatibility endpoints
    fn networks_compat(&self) -> &dyn apis::NetworksCompat;
    /// Actions related to pods
    fn pods(&self) -> &dyn apis::Pods;
    /// Actions related to secrets
    fn secrets(&self) -> &dyn apis::Secrets;
    /// Actions related to secrets for the compatibility endpoints
    fn secrets_compat(&self) -> &dyn apis::SecretsCompat;
    /// Actions related to Podman engine
    fn system(&self) -> &dyn apis::System;
    /// Actions related to Podman and compatibility engines
    fn system_compat(&self) -> &dyn apis::SystemCompat;
    /// Actions related to volumes
    fn volumes(&self) -> &dyn apis::Volumes;
    /// Actions related to volumes for the compatibility endpoints
    fn volumes_compat(&self) -> &dyn apis::VolumesCompat;
}
#[macro_export]
macro_rules! impl_crate_v5_traits {
    ($struct_name:ident) => {
        impl crate::v5::Client for $struct_name {
            #[doc = " Actions related to containers"]
            fn containers(&self) -> &dyn crate::v5::apis::Containers {
                self
            }
            #[doc = " Actions related to containers for the compatibility endpoints"]
            fn containers_compat(&self) -> &dyn crate::v5::apis::ContainersCompat {
                self
            }
            #[doc = " Actions related to exec"]
            fn exec(&self) -> &dyn crate::v5::apis::Exec {
                self
            }
            #[doc = " Actions related to exec for the compatibility endpoints"]
            fn exec_compat(&self) -> &dyn crate::v5::apis::ExecCompat {
                self
            }
            #[doc = " Actions related to images"]
            fn images(&self) -> &dyn crate::v5::apis::Images {
                self
            }
            #[doc = " Actions related to images for the compatibility endpoints"]
            fn images_compat(&self) -> &dyn crate::v5::apis::ImagesCompat {
                self
            }
            #[doc = " Actions related to manifests"]
            fn manifests(&self) -> &dyn crate::v5::apis::Manifests {
                self
            }
            #[doc = " Actions related to networks"]
            fn networks(&self) -> &dyn crate::v5::apis::Networks {
                self
            }
            #[doc = " Actions related to networks for the compatibility endpoints"]
            fn networks_compat(&self) -> &dyn crate::v5::apis::NetworksCompat {
                self
            }
            #[doc = " Actions related to pods"]
            fn pods(&self) -> &dyn crate::v5::apis::Pods {
                self
            }
            #[doc = " Actions related to secrets"]
            fn secrets(&self) -> &dyn crate::v5::apis::Secrets {
                self
            }
            #[doc = " Actions related to secrets for the compatibility endpoints"]
            fn secrets_compat(&self) -> &dyn crate::v5::apis::SecretsCompat {
                self
            }
            #[doc = " Actions related to Podman engine"]
            fn system(&self) -> &dyn crate::v5::apis::System {
                self
            }
            #[doc = " Actions related to Podman and compatibility engines"]
            fn system_compat(&self) -> &dyn crate::v5::apis::SystemCompat {
                self
            }
            #[doc = " Actions related to volumes"]
            fn volumes(&self) -> &dyn crate::v5::apis::Volumes {
                self
            }
            #[doc = " Actions related to volumes for the compatibility endpoints"]
            fn volumes_compat(&self) -> &dyn crate::v5::apis::VolumesCompat {
                self
            }
        }
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

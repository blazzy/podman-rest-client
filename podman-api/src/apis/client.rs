use std::rc::Rc;

use super::configuration::Configuration;
use hyper;
use hyper_util::client::legacy::connect::Connect;

pub struct APIClient {
    containers_api: Box<dyn crate::apis::ContainersApi>,
    containers_compat_api: Box<dyn crate::apis::ContainersCompatApi>,
    exec_api: Box<dyn crate::apis::ExecApi>,
    exec_compat_api: Box<dyn crate::apis::ExecCompatApi>,
    images_api: Box<dyn crate::apis::ImagesApi>,
    images_compat_api: Box<dyn crate::apis::ImagesCompatApi>,
    manifests_api: Box<dyn crate::apis::ManifestsApi>,
    networks_api: Box<dyn crate::apis::NetworksApi>,
    networks_compat_api: Box<dyn crate::apis::NetworksCompatApi>,
    pods_api: Box<dyn crate::apis::PodsApi>,
    secrets_api: Box<dyn crate::apis::SecretsApi>,
    secrets_compat_api: Box<dyn crate::apis::SecretsCompatApi>,
    system_api: Box<dyn crate::apis::SystemApi>,
    system_compat_api: Box<dyn crate::apis::SystemCompatApi>,
    volumes_api: Box<dyn crate::apis::VolumesApi>,
    volumes_compat_api: Box<dyn crate::apis::VolumesCompatApi>,
}

impl APIClient {
    pub fn new<C: Connect>(configuration: Configuration<C>) -> APIClient
    where
        C: Clone + std::marker::Send + Sync + 'static,
    {
        let rc = Rc::new(configuration);

        APIClient {
            containers_api: Box::new(crate::apis::ContainersApiClient::new(rc.clone())),
            containers_compat_api: Box::new(crate::apis::ContainersCompatApiClient::new(
                rc.clone(),
            )),
            exec_api: Box::new(crate::apis::ExecApiClient::new(rc.clone())),
            exec_compat_api: Box::new(crate::apis::ExecCompatApiClient::new(rc.clone())),
            images_api: Box::new(crate::apis::ImagesApiClient::new(rc.clone())),
            images_compat_api: Box::new(crate::apis::ImagesCompatApiClient::new(rc.clone())),
            manifests_api: Box::new(crate::apis::ManifestsApiClient::new(rc.clone())),
            networks_api: Box::new(crate::apis::NetworksApiClient::new(rc.clone())),
            networks_compat_api: Box::new(crate::apis::NetworksCompatApiClient::new(rc.clone())),
            pods_api: Box::new(crate::apis::PodsApiClient::new(rc.clone())),
            secrets_api: Box::new(crate::apis::SecretsApiClient::new(rc.clone())),
            secrets_compat_api: Box::new(crate::apis::SecretsCompatApiClient::new(rc.clone())),
            system_api: Box::new(crate::apis::SystemApiClient::new(rc.clone())),
            system_compat_api: Box::new(crate::apis::SystemCompatApiClient::new(rc.clone())),
            volumes_api: Box::new(crate::apis::VolumesApiClient::new(rc.clone())),
            volumes_compat_api: Box::new(crate::apis::VolumesCompatApiClient::new(rc.clone())),
        }
    }

    pub fn containers_api(&self) -> &dyn crate::apis::ContainersApi {
        self.containers_api.as_ref()
    }

    pub fn containers_compat_api(&self) -> &dyn crate::apis::ContainersCompatApi {
        self.containers_compat_api.as_ref()
    }

    pub fn exec_api(&self) -> &dyn crate::apis::ExecApi {
        self.exec_api.as_ref()
    }

    pub fn exec_compat_api(&self) -> &dyn crate::apis::ExecCompatApi {
        self.exec_compat_api.as_ref()
    }

    pub fn images_api(&self) -> &dyn crate::apis::ImagesApi {
        self.images_api.as_ref()
    }

    pub fn images_compat_api(&self) -> &dyn crate::apis::ImagesCompatApi {
        self.images_compat_api.as_ref()
    }

    pub fn manifests_api(&self) -> &dyn crate::apis::ManifestsApi {
        self.manifests_api.as_ref()
    }

    pub fn networks_api(&self) -> &dyn crate::apis::NetworksApi {
        self.networks_api.as_ref()
    }

    pub fn networks_compat_api(&self) -> &dyn crate::apis::NetworksCompatApi {
        self.networks_compat_api.as_ref()
    }

    pub fn pods_api(&self) -> &dyn crate::apis::PodsApi {
        self.pods_api.as_ref()
    }

    pub fn secrets_api(&self) -> &dyn crate::apis::SecretsApi {
        self.secrets_api.as_ref()
    }

    pub fn secrets_compat_api(&self) -> &dyn crate::apis::SecretsCompatApi {
        self.secrets_compat_api.as_ref()
    }

    pub fn system_api(&self) -> &dyn crate::apis::SystemApi {
        self.system_api.as_ref()
    }

    pub fn system_compat_api(&self) -> &dyn crate::apis::SystemCompatApi {
        self.system_compat_api.as_ref()
    }

    pub fn volumes_api(&self) -> &dyn crate::apis::VolumesApi {
        self.volumes_api.as_ref()
    }

    pub fn volumes_compat_api(&self) -> &dyn crate::apis::VolumesCompatApi {
        self.volumes_compat_api.as_ref()
    }
}

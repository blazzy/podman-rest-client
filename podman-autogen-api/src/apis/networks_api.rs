/*
 * supports a RESTful API for the Libpod library
 *
 * This documentation describes the Podman v2.x+ RESTful API. It consists of a Docker-compatible API and a Libpod API providing support for Podman’s unique features such as pods.  To start the service and keep it running for 5,000 seconds (-t 0 runs forever):  podman system service -t 5000 &  You can then use cURL on the socket using requests documented below.  NOTE: if you install the package podman-docker, it will create a symbolic link for /run/docker.sock to /run/podman/podman.sock  NOTE: Some fields in the API response JSON are encoded as omitempty, which means that if said field has a zero value, they will not be encoded in the API response. This is a feature to help reduce the size of the JSON responses returned via the API.  NOTE: Due to the limitations of [go-swagger](https://github.com/go-swagger/go-swagger), some field values that have a complex type show up as null in the docs as well as in the API responses. This is because the zero value for the field type is null. The field description in the docs will state what type the field is expected to be for such cases.  See podman-system-service(1) for more information.  Quick Examples:  'podman info'  curl --unix-socket /run/podman/podman.sock http://d/v5.0.0/libpod/info  'podman pull quay.io/containers/podman'  curl -XPOST --unix-socket /run/podman/podman.sock -v 'http://d/v5.0.0/images/create?fromImage=quay.io%2Fcontainers%2Fpodman'  'podman list images'  curl --unix-socket /run/podman/podman.sock -v 'http://d/v5.0.0/libpod/images/json' | jq
 *
 * The version of the OpenAPI document: 5.0.0
 * Contact: podman@lists.podman.io
 * Generated by: https://openapi-generator.tech
 */

use std::borrow::Borrow;
#[allow(unused_imports)]
use std::option::Option;
use std::pin::Pin;
use std::sync::Arc;

use futures::Future;
use hyper;
use hyper_util::client::legacy::connect::Connect;

use super::request as __internal_request;
use super::{configuration, Error};
use crate::models;

pub struct NetworksApiClient<C: Connect>
where
    C: Clone + std::marker::Send + Sync + 'static,
{
    configuration: Arc<configuration::Configuration<C>>,
}

impl<C: Connect> NetworksApiClient<C>
where
    C: Clone + std::marker::Send + Sync,
{
    pub fn new(configuration: Arc<configuration::Configuration<C>>) -> NetworksApiClient<C> {
        NetworksApiClient { configuration }
    }
}

pub trait NetworksApi: Send + Sync {
    fn network_connect_libpod(
        &self,
        name: &str,
        create: Option<models::NetworkConnectOptions>,
    ) -> Pin<Box<dyn Future<Output = Result<(), Error>> + Send>>;
    fn network_create_libpod(
        &self,
        create: Option<models::NetworkCreateLibpod>,
    ) -> Pin<Box<dyn Future<Output = Result<models::Network, Error>> + Send>>;
    fn network_delete_libpod(
        &self,
        name: &str,
        force: Option<bool>,
    ) -> Pin<Box<dyn Future<Output = Result<Vec<models::NetworkRmReport>, Error>> + Send>>;
    fn network_disconnect_libpod(
        &self,
        name: &str,
        create: Option<models::NetworkDisconnect>,
    ) -> Pin<Box<dyn Future<Output = Result<(), Error>> + Send>>;
    fn network_exists_libpod(
        &self,
        name: &str,
    ) -> Pin<Box<dyn Future<Output = Result<(), Error>> + Send>>;
    fn network_inspect_libpod(
        &self,
        name: &str,
    ) -> Pin<Box<dyn Future<Output = Result<models::NetworkInspectReport, Error>> + Send>>;
    fn network_list_libpod(
        &self,
        filters: Option<&str>,
    ) -> Pin<Box<dyn Future<Output = Result<Vec<models::Network>, Error>> + Send>>;
    fn network_prune_libpod(
        &self,
        filters: Option<&str>,
    ) -> Pin<Box<dyn Future<Output = Result<Vec<models::NetworkPruneReport>, Error>> + Send>>;
    fn network_update_libpod(
        &self,
        name: &str,
        update: Option<models::NetworkUpdateOptions>,
    ) -> Pin<Box<dyn Future<Output = Result<(), Error>> + Send>>;
}

impl<C: Connect> NetworksApi for NetworksApiClient<C>
where
    C: Clone + std::marker::Send + Sync,
{
    #[allow(unused_mut)]
    fn network_connect_libpod(
        &self,
        name: &str,
        create: Option<models::NetworkConnectOptions>,
    ) -> Pin<Box<dyn Future<Output = Result<(), Error>> + Send>> {
        let mut req = __internal_request::Request::new(
            hyper::Method::POST,
            "/libpod/networks/{name}/connect".to_string(),
        );
        req = req.with_path_param("name".to_string(), name.to_string());
        req = req.with_body_param(create);
        req = req.returns_nothing();

        req.execute(self.configuration.borrow())
    }

    #[allow(unused_mut)]
    fn network_create_libpod(
        &self,
        create: Option<models::NetworkCreateLibpod>,
    ) -> Pin<Box<dyn Future<Output = Result<models::Network, Error>> + Send>> {
        let mut req = __internal_request::Request::new(
            hyper::Method::POST,
            "/libpod/networks/create".to_string(),
        );
        req = req.with_body_param(create);

        req.execute(self.configuration.borrow())
    }

    #[allow(unused_mut)]
    fn network_delete_libpod(
        &self,
        name: &str,
        force: Option<bool>,
    ) -> Pin<Box<dyn Future<Output = Result<Vec<models::NetworkRmReport>, Error>> + Send>> {
        let mut req = __internal_request::Request::new(
            hyper::Method::DELETE,
            "/libpod/networks/{name}".to_string(),
        );
        if let Some(ref s) = force {
            let query_value = s.to_string();
            req = req.with_query_param("force".to_string(), query_value);
        }
        req = req.with_path_param("name".to_string(), name.to_string());

        req.execute(self.configuration.borrow())
    }

    #[allow(unused_mut)]
    fn network_disconnect_libpod(
        &self,
        name: &str,
        create: Option<models::NetworkDisconnect>,
    ) -> Pin<Box<dyn Future<Output = Result<(), Error>> + Send>> {
        let mut req = __internal_request::Request::new(
            hyper::Method::POST,
            "/libpod/networks/{name}/disconnect".to_string(),
        );
        req = req.with_path_param("name".to_string(), name.to_string());
        req = req.with_body_param(create);
        req = req.returns_nothing();

        req.execute(self.configuration.borrow())
    }

    #[allow(unused_mut)]
    fn network_exists_libpod(
        &self,
        name: &str,
    ) -> Pin<Box<dyn Future<Output = Result<(), Error>> + Send>> {
        let mut req = __internal_request::Request::new(
            hyper::Method::GET,
            "/libpod/networks/{name}/exists".to_string(),
        );
        req = req.with_path_param("name".to_string(), name.to_string());
        req = req.returns_nothing();

        req.execute(self.configuration.borrow())
    }

    #[allow(unused_mut)]
    fn network_inspect_libpod(
        &self,
        name: &str,
    ) -> Pin<Box<dyn Future<Output = Result<models::NetworkInspectReport, Error>> + Send>> {
        let mut req = __internal_request::Request::new(
            hyper::Method::GET,
            "/libpod/networks/{name}/json".to_string(),
        );
        req = req.with_path_param("name".to_string(), name.to_string());

        req.execute(self.configuration.borrow())
    }

    #[allow(unused_mut)]
    fn network_list_libpod(
        &self,
        filters: Option<&str>,
    ) -> Pin<Box<dyn Future<Output = Result<Vec<models::Network>, Error>> + Send>> {
        let mut req = __internal_request::Request::new(
            hyper::Method::GET,
            "/libpod/networks/json".to_string(),
        );
        if let Some(ref s) = filters {
            let query_value = s.to_string();
            req = req.with_query_param("filters".to_string(), query_value);
        }

        req.execute(self.configuration.borrow())
    }

    #[allow(unused_mut)]
    fn network_prune_libpod(
        &self,
        filters: Option<&str>,
    ) -> Pin<Box<dyn Future<Output = Result<Vec<models::NetworkPruneReport>, Error>> + Send>> {
        let mut req = __internal_request::Request::new(
            hyper::Method::POST,
            "/libpod/networks/prune".to_string(),
        );
        if let Some(ref s) = filters {
            let query_value = s.to_string();
            req = req.with_query_param("filters".to_string(), query_value);
        }

        req.execute(self.configuration.borrow())
    }

    #[allow(unused_mut)]
    fn network_update_libpod(
        &self,
        name: &str,
        update: Option<models::NetworkUpdateOptions>,
    ) -> Pin<Box<dyn Future<Output = Result<(), Error>> + Send>> {
        let mut req = __internal_request::Request::new(
            hyper::Method::POST,
            "/libpod/networks/{name}/update".to_string(),
        );
        req = req.with_path_param("name".to_string(), name.to_string());
        req = req.with_body_param(update);
        req = req.returns_nothing();

        req.execute(self.configuration.borrow())
    }
}

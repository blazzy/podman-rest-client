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

pub struct NetworksCompatApiClient<C: Connect>
where
    C: Clone + std::marker::Send + Sync + 'static,
{
    configuration: Arc<configuration::Configuration<C>>,
}

impl<C: Connect> NetworksCompatApiClient<C>
where
    C: Clone + std::marker::Send + Sync,
{
    pub fn new(configuration: Arc<configuration::Configuration<C>>) -> NetworksCompatApiClient<C> {
        NetworksCompatApiClient { configuration }
    }
}

pub trait NetworksCompatApi: Send {
    fn network_connect(
        &self,
        name: &str,
        create: Option<models::NetworkConnect>,
    ) -> Pin<Box<dyn Future<Output = Result<(), Error>> + Send>>;
    fn network_create(
        &self,
        create: Option<models::NetworkCreateRequest>,
    ) -> Pin<Box<dyn Future<Output = Result<models::NetworkCreate201Response, Error>> + Send>>;
    fn network_delete(&self, name: &str)
        -> Pin<Box<dyn Future<Output = Result<(), Error>> + Send>>;
    fn network_disconnect(
        &self,
        name: &str,
        create: Option<models::NetworkDisconnect>,
    ) -> Pin<Box<dyn Future<Output = Result<(), Error>> + Send>>;
    fn network_inspect(
        &self,
        name: &str,
        verbose: Option<bool>,
        scope: Option<&str>,
    ) -> Pin<Box<dyn Future<Output = Result<models::NetworkResource, Error>> + Send>>;
    fn network_list(
        &self,
        filters: Option<&str>,
    ) -> Pin<Box<dyn Future<Output = Result<Vec<models::NetworkResource>, Error>> + Send>>;
    fn network_prune(
        &self,
        filters: Option<&str>,
    ) -> Pin<Box<dyn Future<Output = Result<models::NetworkPrune200Response, Error>> + Send>>;
}

impl<C: Connect> NetworksCompatApi for NetworksCompatApiClient<C>
where
    C: Clone + std::marker::Send + Sync,
{
    #[allow(unused_mut)]
    fn network_connect(
        &self,
        name: &str,
        create: Option<models::NetworkConnect>,
    ) -> Pin<Box<dyn Future<Output = Result<(), Error>> + Send>> {
        let mut req = __internal_request::Request::new(
            hyper::Method::POST,
            "/networks/{name}/connect".to_string(),
        );
        req = req.with_path_param("name".to_string(), name.to_string());
        req = req.with_body_param(create);
        req = req.returns_nothing();

        req.execute(self.configuration.borrow())
    }

    #[allow(unused_mut)]
    fn network_create(
        &self,
        create: Option<models::NetworkCreateRequest>,
    ) -> Pin<Box<dyn Future<Output = Result<models::NetworkCreate201Response, Error>> + Send>> {
        let mut req =
            __internal_request::Request::new(hyper::Method::POST, "/networks/create".to_string());
        req = req.with_body_param(create);

        req.execute(self.configuration.borrow())
    }

    #[allow(unused_mut)]
    fn network_delete(
        &self,
        name: &str,
    ) -> Pin<Box<dyn Future<Output = Result<(), Error>> + Send>> {
        let mut req =
            __internal_request::Request::new(hyper::Method::DELETE, "/networks/{name}".to_string());
        req = req.with_path_param("name".to_string(), name.to_string());
        req = req.returns_nothing();

        req.execute(self.configuration.borrow())
    }

    #[allow(unused_mut)]
    fn network_disconnect(
        &self,
        name: &str,
        create: Option<models::NetworkDisconnect>,
    ) -> Pin<Box<dyn Future<Output = Result<(), Error>> + Send>> {
        let mut req = __internal_request::Request::new(
            hyper::Method::POST,
            "/networks/{name}/disconnect".to_string(),
        );
        req = req.with_path_param("name".to_string(), name.to_string());
        req = req.with_body_param(create);
        req = req.returns_nothing();

        req.execute(self.configuration.borrow())
    }

    #[allow(unused_mut)]
    fn network_inspect(
        &self,
        name: &str,
        verbose: Option<bool>,
        scope: Option<&str>,
    ) -> Pin<Box<dyn Future<Output = Result<models::NetworkResource, Error>> + Send>> {
        let mut req =
            __internal_request::Request::new(hyper::Method::GET, "/networks/{name}".to_string());
        if let Some(ref s) = verbose {
            let query_value = s.to_string();
            req = req.with_query_param("verbose".to_string(), query_value);
        }
        if let Some(ref s) = scope {
            let query_value = s.to_string();
            req = req.with_query_param("scope".to_string(), query_value);
        }
        req = req.with_path_param("name".to_string(), name.to_string());

        req.execute(self.configuration.borrow())
    }

    #[allow(unused_mut)]
    fn network_list(
        &self,
        filters: Option<&str>,
    ) -> Pin<Box<dyn Future<Output = Result<Vec<models::NetworkResource>, Error>> + Send>> {
        let mut req = __internal_request::Request::new(hyper::Method::GET, "/networks".to_string());
        if let Some(ref s) = filters {
            let query_value = s.to_string();
            req = req.with_query_param("filters".to_string(), query_value);
        }

        req.execute(self.configuration.borrow())
    }

    #[allow(unused_mut)]
    fn network_prune(
        &self,
        filters: Option<&str>,
    ) -> Pin<Box<dyn Future<Output = Result<models::NetworkPrune200Response, Error>> + Send>> {
        let mut req =
            __internal_request::Request::new(hyper::Method::POST, "/networks/prune".to_string());
        if let Some(ref s) = filters {
            let query_value = s.to_string();
            req = req.with_query_param("filters".to_string(), query_value);
        }

        req.execute(self.configuration.borrow())
    }
}

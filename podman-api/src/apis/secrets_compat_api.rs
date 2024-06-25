/*
 * supports a RESTful API for the Libpod library
 *
 * This documentation describes the Podman v2.x+ RESTful API. It consists of a Docker-compatible API and a Libpod API providing support for Podman’s unique features such as pods.  To start the service and keep it running for 5,000 seconds (-t 0 runs forever):  podman system service -t 5000 &  You can then use cURL on the socket using requests documented below.  NOTE: if you install the package podman-docker, it will create a symbolic link for /run/docker.sock to /run/podman/podman.sock  NOTE: Some fields in the API response JSON are encoded as omitempty, which means that if said field has a zero value, they will not be encoded in the API response. This is a feature to help reduce the size of the JSON responses returned via the API.  NOTE: Due to the limitations of [go-swagger](https://github.com/go-swagger/go-swagger), some field values that have a complex type show up as null in the docs as well as in the API responses. This is because the zero value for the field type is null. The field description in the docs will state what type the field is expected to be for such cases.  See podman-system-service(1) for more information.  Quick Examples:  'podman info'  curl --unix-socket /run/podman/podman.sock http://d/v5.0.0/libpod/info  'podman pull quay.io/containers/podman'  curl -XPOST --unix-socket /run/podman/podman.sock -v 'http://d/v5.0.0/images/create?fromImage=quay.io%2Fcontainers%2Fpodman'  'podman list images'  curl --unix-socket /run/podman/podman.sock -v 'http://d/v5.0.0/libpod/images/json' | jq
 *
 * The version of the OpenAPI document: 5.0.0
 * Contact: podman@lists.podman.io
 * Generated by: https://openapi-generator.tech
 */

use std::rc::Rc;
use std::borrow::Borrow;
use std::pin::Pin;
#[allow(unused_imports)]
use std::option::Option;

use hyper;
use hyper_util::client::legacy::connect::Connect;
use futures::Future;

use crate::models;
use super::{Error, configuration};
use super::request as __internal_request;

pub struct SecretsCompatApiClient<C: Connect>
    where C: Clone + std::marker::Send + Sync + 'static {
    configuration: Rc<configuration::Configuration<C>>,
}

impl<C: Connect> SecretsCompatApiClient<C>
    where C: Clone + std::marker::Send + Sync {
    pub fn new(configuration: Rc<configuration::Configuration<C>>) -> SecretsCompatApiClient<C> {
        SecretsCompatApiClient {
            configuration,
        }
    }
}

pub trait SecretsCompatApi {
    fn secret_create(&self, create: Option<models::SecretCreate>) -> Pin<Box<dyn Future<Output = Result<models::SecretCreateLibpod201Response, Error>>>>;
    fn secret_delete(&self, name: &str) -> Pin<Box<dyn Future<Output = Result<(), Error>>>>;
    fn secret_inspect(&self, name: &str) -> Pin<Box<dyn Future<Output = Result<models::SecretInfoReportCompat, Error>>>>;
    fn secret_list(&self, filters: Option<&str>) -> Pin<Box<dyn Future<Output = Result<Vec<models::SecretInfoReportCompat>, Error>>>>;
}

impl<C: Connect>SecretsCompatApi for SecretsCompatApiClient<C>
    where C: Clone + std::marker::Send + Sync {
    #[allow(unused_mut)]
    fn secret_create(&self, create: Option<models::SecretCreate>) -> Pin<Box<dyn Future<Output = Result<models::SecretCreateLibpod201Response, Error>>>> {
        let mut req = __internal_request::Request::new(hyper::Method::POST, "/secrets/create".to_string())
        ;
        req = req.with_body_param(create);

        req.execute(self.configuration.borrow())
    }

    #[allow(unused_mut)]
    fn secret_delete(&self, name: &str) -> Pin<Box<dyn Future<Output = Result<(), Error>>>> {
        let mut req = __internal_request::Request::new(hyper::Method::DELETE, "/secrets/{name}".to_string())
        ;
        req = req.with_path_param("name".to_string(), name.to_string());
        req = req.returns_nothing();

        req.execute(self.configuration.borrow())
    }

    #[allow(unused_mut)]
    fn secret_inspect(&self, name: &str) -> Pin<Box<dyn Future<Output = Result<models::SecretInfoReportCompat, Error>>>> {
        let mut req = __internal_request::Request::new(hyper::Method::GET, "/secrets/{name}".to_string())
        ;
        req = req.with_path_param("name".to_string(), name.to_string());

        req.execute(self.configuration.borrow())
    }

    #[allow(unused_mut)]
    fn secret_list(&self, filters: Option<&str>) -> Pin<Box<dyn Future<Output = Result<Vec<models::SecretInfoReportCompat>, Error>>>> {
        let mut req = __internal_request::Request::new(hyper::Method::GET, "/secrets".to_string())
        ;
        if let Some(ref s) = filters {
            let query_value = s.to_string();
            req = req.with_query_param("filters".to_string(), query_value);
        }

        req.execute(self.configuration.borrow())
    }

}

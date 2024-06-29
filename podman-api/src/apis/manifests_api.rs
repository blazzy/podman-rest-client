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
use std::rc::Rc;

use futures::Future;
use hyper;
use hyper_util::client::legacy::connect::Connect;

use super::request as __internal_request;
use super::{configuration, Error};
use crate::models;

pub struct ManifestsApiClient<C: Connect>
where
    C: Clone + std::marker::Send + Sync + 'static,
{
    configuration: Rc<configuration::Configuration<C>>,
}

impl<C: Connect> ManifestsApiClient<C>
where
    C: Clone + std::marker::Send + Sync,
{
    pub fn new(configuration: Rc<configuration::Configuration<C>>) -> ManifestsApiClient<C> {
        ManifestsApiClient { configuration }
    }
}

pub trait ManifestsApi {
    fn manifest_add_libpod(
        &self,
        name: &str,
        options: Option<models::ManifestAddOptions>,
    ) -> Pin<Box<dyn Future<Output = Result<models::IdResponse, Error>>>>;
    fn manifest_create_libpod(
        &self,
        name: &str,
        images: &str,
        all: Option<bool>,
        amend: Option<bool>,
        options: Option<models::ManifestModifyOptions>,
    ) -> Pin<Box<dyn Future<Output = Result<models::IdResponse, Error>>>>;
    fn manifest_delete_libpod(
        &self,
        name: &str,
    ) -> Pin<Box<dyn Future<Output = Result<models::LibpodImagesRemoveReport, Error>>>>;
    fn manifest_exists_libpod(
        &self,
        name: &str,
    ) -> Pin<Box<dyn Future<Output = Result<(), Error>>>>;
    fn manifest_inspect_libpod(
        &self,
        name: &str,
        tls_verify: Option<bool>,
    ) -> Pin<Box<dyn Future<Output = Result<models::Schema2ListPublic, Error>>>>;
    fn manifest_modify_libpod(
        &self,
        name: &str,
        options: models::ManifestModifyOptions,
        tls_verify: Option<bool>,
    ) -> Pin<Box<dyn Future<Output = Result<models::ManifestModifyReport, Error>>>>;
    fn manifest_push_libpod(
        &self,
        name: &str,
        destination: &str,
        add_compression: Option<Vec<String>>,
        force_compression_format: Option<bool>,
        all: Option<bool>,
        tls_verify: Option<bool>,
        quiet: Option<bool>,
    ) -> Pin<Box<dyn Future<Output = Result<models::IdResponse, Error>>>>;
    fn manifest_push_v3_libpod(
        &self,
        name: &str,
        destination: &str,
        all: Option<bool>,
    ) -> Pin<Box<dyn Future<Output = Result<models::IdResponse, Error>>>>;
}

impl<C: Connect> ManifestsApi for ManifestsApiClient<C>
where
    C: Clone + std::marker::Send + Sync,
{
    #[allow(unused_mut)]
    fn manifest_add_libpod(
        &self,
        name: &str,
        options: Option<models::ManifestAddOptions>,
    ) -> Pin<Box<dyn Future<Output = Result<models::IdResponse, Error>>>> {
        let mut req = __internal_request::Request::new(
            hyper::Method::POST,
            "/libpod/manifests/{name}/add".to_string(),
        );
        req = req.with_path_param("name".to_string(), name.to_string());
        req = req.with_body_param(options);

        req.execute(self.configuration.borrow())
    }

    #[allow(unused_mut)]
    fn manifest_create_libpod(
        &self,
        name: &str,
        images: &str,
        all: Option<bool>,
        amend: Option<bool>,
        options: Option<models::ManifestModifyOptions>,
    ) -> Pin<Box<dyn Future<Output = Result<models::IdResponse, Error>>>> {
        let mut req = __internal_request::Request::new(
            hyper::Method::POST,
            "/libpod/manifests/{name}".to_string(),
        );
        req = req.with_query_param("images".to_string(), images.to_string());
        if let Some(ref s) = all {
            let query_value = s.to_string();
            req = req.with_query_param("all".to_string(), query_value);
        }
        if let Some(ref s) = amend {
            let query_value = s.to_string();
            req = req.with_query_param("amend".to_string(), query_value);
        }
        req = req.with_path_param("name".to_string(), name.to_string());
        req = req.with_body_param(options);

        req.execute(self.configuration.borrow())
    }

    #[allow(unused_mut)]
    fn manifest_delete_libpod(
        &self,
        name: &str,
    ) -> Pin<Box<dyn Future<Output = Result<models::LibpodImagesRemoveReport, Error>>>> {
        let mut req = __internal_request::Request::new(
            hyper::Method::DELETE,
            "/libpod/manifests/{name}".to_string(),
        );
        req = req.with_path_param("name".to_string(), name.to_string());

        req.execute(self.configuration.borrow())
    }

    #[allow(unused_mut)]
    fn manifest_exists_libpod(
        &self,
        name: &str,
    ) -> Pin<Box<dyn Future<Output = Result<(), Error>>>> {
        let mut req = __internal_request::Request::new(
            hyper::Method::GET,
            "/libpod/manifests/{name}/exists".to_string(),
        );
        req = req.with_path_param("name".to_string(), name.to_string());
        req = req.returns_nothing();

        req.execute(self.configuration.borrow())
    }

    #[allow(unused_mut)]
    fn manifest_inspect_libpod(
        &self,
        name: &str,
        tls_verify: Option<bool>,
    ) -> Pin<Box<dyn Future<Output = Result<models::Schema2ListPublic, Error>>>> {
        let mut req = __internal_request::Request::new(
            hyper::Method::GET,
            "/libpod/manifests/{name}/json".to_string(),
        );
        if let Some(ref s) = tls_verify {
            let query_value = s.to_string();
            req = req.with_query_param("tlsVerify".to_string(), query_value);
        }
        req = req.with_path_param("name".to_string(), name.to_string());

        req.execute(self.configuration.borrow())
    }

    #[allow(unused_mut)]
    fn manifest_modify_libpod(
        &self,
        name: &str,
        options: models::ManifestModifyOptions,
        tls_verify: Option<bool>,
    ) -> Pin<Box<dyn Future<Output = Result<models::ManifestModifyReport, Error>>>> {
        let mut req = __internal_request::Request::new(
            hyper::Method::PUT,
            "/libpod/manifests/{name}".to_string(),
        );
        if let Some(ref s) = tls_verify {
            let query_value = s.to_string();
            req = req.with_query_param("tlsVerify".to_string(), query_value);
        }
        req = req.with_path_param("name".to_string(), name.to_string());
        req = req.with_body_param(options);

        req.execute(self.configuration.borrow())
    }

    #[allow(unused_mut)]
    fn manifest_push_libpod(
        &self,
        name: &str,
        destination: &str,
        add_compression: Option<Vec<String>>,
        force_compression_format: Option<bool>,
        all: Option<bool>,
        tls_verify: Option<bool>,
        quiet: Option<bool>,
    ) -> Pin<Box<dyn Future<Output = Result<models::IdResponse, Error>>>> {
        let mut req = __internal_request::Request::new(
            hyper::Method::POST,
            "/libpod/manifests/{name}/registry/{destination}".to_string(),
        );
        if let Some(ref s) = add_compression {
            let query_value = s
                .iter()
                .map(|s| s.to_string())
                .collect::<Vec<String>>()
                .join(",");
            req = req.with_query_param("addCompression".to_string(), query_value);
        }
        if let Some(ref s) = force_compression_format {
            let query_value = s.to_string();
            req = req.with_query_param("forceCompressionFormat".to_string(), query_value);
        }
        if let Some(ref s) = all {
            let query_value = s.to_string();
            req = req.with_query_param("all".to_string(), query_value);
        }
        if let Some(ref s) = tls_verify {
            let query_value = s.to_string();
            req = req.with_query_param("tlsVerify".to_string(), query_value);
        }
        if let Some(ref s) = quiet {
            let query_value = s.to_string();
            req = req.with_query_param("quiet".to_string(), query_value);
        }
        req = req.with_path_param("name".to_string(), name.to_string());
        req = req.with_path_param("destination".to_string(), destination.to_string());

        req.execute(self.configuration.borrow())
    }

    #[allow(unused_mut)]
    fn manifest_push_v3_libpod(
        &self,
        name: &str,
        destination: &str,
        all: Option<bool>,
    ) -> Pin<Box<dyn Future<Output = Result<models::IdResponse, Error>>>> {
        let mut req = __internal_request::Request::new(
            hyper::Method::POST,
            "/libpod/manifests/{name}/push".to_string(),
        );
        req = req.with_query_param("destination".to_string(), destination.to_string());
        if let Some(ref s) = all {
            let query_value = s.to_string();
            req = req.with_query_param("all".to_string(), query_value);
        }
        req = req.with_path_param("name".to_string(), name.to_string());

        req.execute(self.configuration.borrow())
    }
}

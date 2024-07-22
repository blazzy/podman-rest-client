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

pub struct ImagesApiClient<C: Connect>
where
    C: Clone + std::marker::Send + Sync + 'static,
{
    configuration: Arc<configuration::Configuration<C>>,
}

impl<C: Connect> ImagesApiClient<C>
where
    C: Clone + std::marker::Send + Sync,
{
    pub fn new(configuration: Arc<configuration::Configuration<C>>) -> ImagesApiClient<C> {
        ImagesApiClient { configuration }
    }
}

pub trait ImagesApi: Send + Sync {
    fn image_build_libpod(
        &self,
        dockerfile: Option<&str>,
        t: Option<&str>,
        allplatforms: Option<bool>,
        extrahosts: Option<&str>,
        remote: Option<&str>,
        q: Option<bool>,
        nocache: Option<bool>,
        cachefrom: Option<&str>,
        pull: Option<bool>,
        rm: Option<bool>,
        forcerm: Option<bool>,
        memory: Option<i32>,
        memswap: Option<i32>,
        cpushares: Option<i32>,
        cpusetcpus: Option<&str>,
        cpuperiod: Option<i32>,
        cpuquota: Option<i32>,
        buildargs: Option<&str>,
        shmsize: Option<i32>,
        squash: Option<bool>,
        labels: Option<&str>,
        layer_label: Option<Vec<String>>,
        layers: Option<bool>,
        networkmode: Option<&str>,
        platform: Option<&str>,
        target: Option<&str>,
        outputs: Option<&str>,
        httpproxy: Option<bool>,
        unsetenv: Option<Vec<String>>,
        unsetlabel: Option<Vec<String>>,
        volume: Option<Vec<String>>,
    ) -> Pin<Box<dyn Future<Output = Result<models::ImageBuildLibpod200Response, Error>> + Send>>;
    fn image_changes_libpod(
        &self,
        name: &str,
        parent: Option<&str>,
        diff_type: Option<&str>,
    ) -> Pin<Box<dyn Future<Output = Result<(), Error>> + Send>>;
    fn image_delete_all_libpod(
        &self,
        images: Option<Vec<String>>,
        all: Option<bool>,
        force: Option<bool>,
        ignore: Option<bool>,
        lookup_manifest: Option<bool>,
    ) -> Pin<Box<dyn Future<Output = Result<models::LibpodImagesRemoveReport, Error>> + Send>>;
    fn image_delete_libpod(
        &self,
        name: &str,
        force: Option<bool>,
    ) -> Pin<Box<dyn Future<Output = Result<models::LibpodImagesRemoveReport, Error>> + Send>>;
    fn image_exists_libpod(
        &self,
        name: &str,
    ) -> Pin<Box<dyn Future<Output = Result<(), Error>> + Send>>;
    fn image_export_libpod(
        &self,
        format: Option<&str>,
        references: Option<Vec<String>>,
        compress: Option<bool>,
        oci_accept_uncompressed_layers: Option<bool>,
    ) -> Pin<Box<dyn Future<Output = Result<std::path::PathBuf, Error>> + Send>>;
    fn image_get_libpod(
        &self,
        name: &str,
        format: Option<&str>,
        compress: Option<bool>,
    ) -> Pin<Box<dyn Future<Output = Result<std::path::PathBuf, Error>> + Send>>;
    fn image_history_libpod(
        &self,
        name: &str,
    ) -> Pin<Box<dyn Future<Output = Result<models::HistoryResponse, Error>> + Send>>;
    fn image_import_libpod(
        &self,
        upload: std::path::PathBuf,
        content_type: Option<&str>,
        changes: Option<Vec<String>>,
        message: Option<&str>,
        reference: Option<&str>,
        url: Option<&str>,
    ) -> Pin<Box<dyn Future<Output = Result<models::ImageImportReport, Error>> + Send>>;
    fn image_inspect_libpod(
        &self,
        name: &str,
    ) -> Pin<Box<dyn Future<Output = Result<models::ImageData, Error>> + Send>>;
    fn image_list_libpod(
        &self,
        all: Option<bool>,
        filters: Option<&str>,
    ) -> Pin<Box<dyn Future<Output = Result<Vec<models::LibpodImageSummary>, Error>> + Send>>;
    fn image_load_libpod(
        &self,
        upload: &str,
    ) -> Pin<Box<dyn Future<Output = Result<models::ImageLoadReport, Error>> + Send>>;
    fn image_prune_libpod(
        &self,
        all: Option<bool>,
        external: Option<bool>,
        filters: Option<&str>,
    ) -> Pin<Box<dyn Future<Output = Result<Vec<models::PruneReport>, Error>> + Send>>;
    fn image_pull_libpod(
        &self,
        reference: Option<&str>,
        quiet: Option<bool>,
        compat_mode: Option<bool>,
        arch: Option<&str>,
        os: Option<&str>,
        variant: Option<&str>,
        policy: Option<&str>,
        tls_verify: Option<bool>,
        all_tags: Option<bool>,
        x_registry_auth: Option<&str>,
    ) -> Pin<Box<dyn Future<Output = Result<models::LibpodImagesPullReport, Error>> + Send>>;
    fn image_push_libpod(
        &self,
        name: &str,
        destination: Option<&str>,
        force_compression_format: Option<bool>,
        tls_verify: Option<bool>,
        quiet: Option<bool>,
        x_registry_auth: Option<&str>,
    ) -> Pin<Box<dyn Future<Output = Result<std::path::PathBuf, Error>> + Send>>;
    fn image_resolve_libpod(
        &self,
        name: &str,
    ) -> Pin<Box<dyn Future<Output = Result<(), Error>> + Send>>;
    fn image_scp_libpod(
        &self,
        name: &str,
        destination: Option<&str>,
        quiet: Option<bool>,
    ) -> Pin<Box<dyn Future<Output = Result<models::ScpReport, Error>> + Send>>;
    fn image_search_libpod(
        &self,
        term: Option<&str>,
        limit: Option<i32>,
        filters: Option<&str>,
        tls_verify: Option<bool>,
        list_tags: Option<bool>,
    ) -> Pin<Box<dyn Future<Output = Result<models::ImageSearch200Response, Error>> + Send>>;
    fn image_tag_libpod(
        &self,
        name: &str,
        repo: Option<&str>,
        tag: Option<&str>,
    ) -> Pin<Box<dyn Future<Output = Result<(), Error>> + Send>>;
    fn image_tree_libpod(
        &self,
        name: &str,
        whatrequires: Option<bool>,
    ) -> Pin<Box<dyn Future<Output = Result<models::ImageTreeReport, Error>> + Send>>;
    fn image_untag_libpod(
        &self,
        name: &str,
        repo: Option<&str>,
        tag: Option<&str>,
    ) -> Pin<Box<dyn Future<Output = Result<(), Error>> + Send>>;
}

impl<C: Connect> ImagesApi for ImagesApiClient<C>
where
    C: Clone + std::marker::Send + Sync,
{
    #[allow(unused_mut)]
    fn image_build_libpod(
        &self,
        dockerfile: Option<&str>,
        t: Option<&str>,
        allplatforms: Option<bool>,
        extrahosts: Option<&str>,
        remote: Option<&str>,
        q: Option<bool>,
        nocache: Option<bool>,
        cachefrom: Option<&str>,
        pull: Option<bool>,
        rm: Option<bool>,
        forcerm: Option<bool>,
        memory: Option<i32>,
        memswap: Option<i32>,
        cpushares: Option<i32>,
        cpusetcpus: Option<&str>,
        cpuperiod: Option<i32>,
        cpuquota: Option<i32>,
        buildargs: Option<&str>,
        shmsize: Option<i32>,
        squash: Option<bool>,
        labels: Option<&str>,
        layer_label: Option<Vec<String>>,
        layers: Option<bool>,
        networkmode: Option<&str>,
        platform: Option<&str>,
        target: Option<&str>,
        outputs: Option<&str>,
        httpproxy: Option<bool>,
        unsetenv: Option<Vec<String>>,
        unsetlabel: Option<Vec<String>>,
        volume: Option<Vec<String>>,
    ) -> Pin<Box<dyn Future<Output = Result<models::ImageBuildLibpod200Response, Error>> + Send>>
    {
        let mut req =
            __internal_request::Request::new(hyper::Method::POST, "/libpod/build".to_string());
        if let Some(ref s) = dockerfile {
            let query_value = s.to_string();
            req = req.with_query_param("dockerfile".to_string(), query_value);
        }
        if let Some(ref s) = t {
            let query_value = s.to_string();
            req = req.with_query_param("t".to_string(), query_value);
        }
        if let Some(ref s) = allplatforms {
            let query_value = s.to_string();
            req = req.with_query_param("allplatforms".to_string(), query_value);
        }
        if let Some(ref s) = extrahosts {
            let query_value = s.to_string();
            req = req.with_query_param("extrahosts".to_string(), query_value);
        }
        if let Some(ref s) = remote {
            let query_value = s.to_string();
            req = req.with_query_param("remote".to_string(), query_value);
        }
        if let Some(ref s) = q {
            let query_value = s.to_string();
            req = req.with_query_param("q".to_string(), query_value);
        }
        if let Some(ref s) = nocache {
            let query_value = s.to_string();
            req = req.with_query_param("nocache".to_string(), query_value);
        }
        if let Some(ref s) = cachefrom {
            let query_value = s.to_string();
            req = req.with_query_param("cachefrom".to_string(), query_value);
        }
        if let Some(ref s) = pull {
            let query_value = s.to_string();
            req = req.with_query_param("pull".to_string(), query_value);
        }
        if let Some(ref s) = rm {
            let query_value = s.to_string();
            req = req.with_query_param("rm".to_string(), query_value);
        }
        if let Some(ref s) = forcerm {
            let query_value = s.to_string();
            req = req.with_query_param("forcerm".to_string(), query_value);
        }
        if let Some(ref s) = memory {
            let query_value = s.to_string();
            req = req.with_query_param("memory".to_string(), query_value);
        }
        if let Some(ref s) = memswap {
            let query_value = s.to_string();
            req = req.with_query_param("memswap".to_string(), query_value);
        }
        if let Some(ref s) = cpushares {
            let query_value = s.to_string();
            req = req.with_query_param("cpushares".to_string(), query_value);
        }
        if let Some(ref s) = cpusetcpus {
            let query_value = s.to_string();
            req = req.with_query_param("cpusetcpus".to_string(), query_value);
        }
        if let Some(ref s) = cpuperiod {
            let query_value = s.to_string();
            req = req.with_query_param("cpuperiod".to_string(), query_value);
        }
        if let Some(ref s) = cpuquota {
            let query_value = s.to_string();
            req = req.with_query_param("cpuquota".to_string(), query_value);
        }
        if let Some(ref s) = buildargs {
            let query_value = s.to_string();
            req = req.with_query_param("buildargs".to_string(), query_value);
        }
        if let Some(ref s) = shmsize {
            let query_value = s.to_string();
            req = req.with_query_param("shmsize".to_string(), query_value);
        }
        if let Some(ref s) = squash {
            let query_value = s.to_string();
            req = req.with_query_param("squash".to_string(), query_value);
        }
        if let Some(ref s) = labels {
            let query_value = s.to_string();
            req = req.with_query_param("labels".to_string(), query_value);
        }
        if let Some(ref s) = layer_label {
            let query_value = s
                .iter()
                .map(|s| s.to_string())
                .collect::<Vec<String>>()
                .join(",");
            req = req.with_query_param("layerLabel".to_string(), query_value);
        }
        if let Some(ref s) = layers {
            let query_value = s.to_string();
            req = req.with_query_param("layers".to_string(), query_value);
        }
        if let Some(ref s) = networkmode {
            let query_value = s.to_string();
            req = req.with_query_param("networkmode".to_string(), query_value);
        }
        if let Some(ref s) = platform {
            let query_value = s.to_string();
            req = req.with_query_param("platform".to_string(), query_value);
        }
        if let Some(ref s) = target {
            let query_value = s.to_string();
            req = req.with_query_param("target".to_string(), query_value);
        }
        if let Some(ref s) = outputs {
            let query_value = s.to_string();
            req = req.with_query_param("outputs".to_string(), query_value);
        }
        if let Some(ref s) = httpproxy {
            let query_value = s.to_string();
            req = req.with_query_param("httpproxy".to_string(), query_value);
        }
        if let Some(ref s) = unsetenv {
            let query_value = s
                .iter()
                .map(|s| s.to_string())
                .collect::<Vec<String>>()
                .join(",");
            req = req.with_query_param("unsetenv".to_string(), query_value);
        }
        if let Some(ref s) = unsetlabel {
            let query_value = s
                .iter()
                .map(|s| s.to_string())
                .collect::<Vec<String>>()
                .join(",");
            req = req.with_query_param("unsetlabel".to_string(), query_value);
        }
        if let Some(ref s) = volume {
            let query_value = s
                .iter()
                .map(|s| s.to_string())
                .collect::<Vec<String>>()
                .join(",");
            req = req.with_query_param("volume".to_string(), query_value);
        }

        req.execute(self.configuration.borrow())
    }

    #[allow(unused_mut)]
    fn image_changes_libpod(
        &self,
        name: &str,
        parent: Option<&str>,
        diff_type: Option<&str>,
    ) -> Pin<Box<dyn Future<Output = Result<(), Error>> + Send>> {
        let mut req = __internal_request::Request::new(
            hyper::Method::GET,
            "/libpod/images/{name}/changes".to_string(),
        );
        if let Some(ref s) = parent {
            let query_value = s.to_string();
            req = req.with_query_param("parent".to_string(), query_value);
        }
        if let Some(ref s) = diff_type {
            let query_value = s.to_string();
            req = req.with_query_param("diffType".to_string(), query_value);
        }
        req = req.with_path_param("name".to_string(), name.to_string());
        req = req.returns_nothing();

        req.execute(self.configuration.borrow())
    }

    #[allow(unused_mut)]
    fn image_delete_all_libpod(
        &self,
        images: Option<Vec<String>>,
        all: Option<bool>,
        force: Option<bool>,
        ignore: Option<bool>,
        lookup_manifest: Option<bool>,
    ) -> Pin<Box<dyn Future<Output = Result<models::LibpodImagesRemoveReport, Error>> + Send>> {
        let mut req = __internal_request::Request::new(
            hyper::Method::DELETE,
            "/libpod/images/remove".to_string(),
        );
        if let Some(ref s) = images {
            let query_value = s
                .iter()
                .map(|s| s.to_string())
                .collect::<Vec<String>>()
                .join(",");
            req = req.with_query_param("images".to_string(), query_value);
        }
        if let Some(ref s) = all {
            let query_value = s.to_string();
            req = req.with_query_param("all".to_string(), query_value);
        }
        if let Some(ref s) = force {
            let query_value = s.to_string();
            req = req.with_query_param("force".to_string(), query_value);
        }
        if let Some(ref s) = ignore {
            let query_value = s.to_string();
            req = req.with_query_param("ignore".to_string(), query_value);
        }
        if let Some(ref s) = lookup_manifest {
            let query_value = s.to_string();
            req = req.with_query_param("lookupManifest".to_string(), query_value);
        }

        req.execute(self.configuration.borrow())
    }

    #[allow(unused_mut)]
    fn image_delete_libpod(
        &self,
        name: &str,
        force: Option<bool>,
    ) -> Pin<Box<dyn Future<Output = Result<models::LibpodImagesRemoveReport, Error>> + Send>> {
        let mut req = __internal_request::Request::new(
            hyper::Method::DELETE,
            "/libpod/images/{name}".to_string(),
        );
        if let Some(ref s) = force {
            let query_value = s.to_string();
            req = req.with_query_param("force".to_string(), query_value);
        }
        req = req.with_path_param("name".to_string(), name.to_string());

        req.execute(self.configuration.borrow())
    }

    #[allow(unused_mut)]
    fn image_exists_libpod(
        &self,
        name: &str,
    ) -> Pin<Box<dyn Future<Output = Result<(), Error>> + Send>> {
        let mut req = __internal_request::Request::new(
            hyper::Method::GET,
            "/libpod/images/{name}/exists".to_string(),
        );
        req = req.with_path_param("name".to_string(), name.to_string());
        req = req.returns_nothing();

        req.execute(self.configuration.borrow())
    }

    #[allow(unused_mut)]
    fn image_export_libpod(
        &self,
        format: Option<&str>,
        references: Option<Vec<String>>,
        compress: Option<bool>,
        oci_accept_uncompressed_layers: Option<bool>,
    ) -> Pin<Box<dyn Future<Output = Result<std::path::PathBuf, Error>> + Send>> {
        let mut req = __internal_request::Request::new(
            hyper::Method::GET,
            "/libpod/images/export".to_string(),
        );
        if let Some(ref s) = format {
            let query_value = s.to_string();
            req = req.with_query_param("format".to_string(), query_value);
        }
        if let Some(ref s) = references {
            let query_value = s
                .iter()
                .map(|s| s.to_string())
                .collect::<Vec<String>>()
                .join(",");
            req = req.with_query_param("references".to_string(), query_value);
        }
        if let Some(ref s) = compress {
            let query_value = s.to_string();
            req = req.with_query_param("compress".to_string(), query_value);
        }
        if let Some(ref s) = oci_accept_uncompressed_layers {
            let query_value = s.to_string();
            req = req.with_query_param("ociAcceptUncompressedLayers".to_string(), query_value);
        }

        req.execute(self.configuration.borrow())
    }

    #[allow(unused_mut)]
    fn image_get_libpod(
        &self,
        name: &str,
        format: Option<&str>,
        compress: Option<bool>,
    ) -> Pin<Box<dyn Future<Output = Result<std::path::PathBuf, Error>> + Send>> {
        let mut req = __internal_request::Request::new(
            hyper::Method::GET,
            "/libpod/images/{name}/get".to_string(),
        );
        if let Some(ref s) = format {
            let query_value = s.to_string();
            req = req.with_query_param("format".to_string(), query_value);
        }
        if let Some(ref s) = compress {
            let query_value = s.to_string();
            req = req.with_query_param("compress".to_string(), query_value);
        }
        req = req.with_path_param("name".to_string(), name.to_string());

        req.execute(self.configuration.borrow())
    }

    #[allow(unused_mut)]
    fn image_history_libpod(
        &self,
        name: &str,
    ) -> Pin<Box<dyn Future<Output = Result<models::HistoryResponse, Error>> + Send>> {
        let mut req = __internal_request::Request::new(
            hyper::Method::GET,
            "/libpod/images/{name}/history".to_string(),
        );
        req = req.with_path_param("name".to_string(), name.to_string());

        req.execute(self.configuration.borrow())
    }

    #[allow(unused_mut)]
    fn image_import_libpod(
        &self,
        upload: std::path::PathBuf,
        content_type: Option<&str>,
        changes: Option<Vec<String>>,
        message: Option<&str>,
        reference: Option<&str>,
        url: Option<&str>,
    ) -> Pin<Box<dyn Future<Output = Result<models::ImageImportReport, Error>> + Send>> {
        let mut req = __internal_request::Request::new(
            hyper::Method::POST,
            "/libpod/images/import".to_string(),
        );
        if let Some(ref s) = changes {
            let query_value = s
                .iter()
                .map(|s| s.to_string())
                .collect::<Vec<String>>()
                .join(",");
            req = req.with_query_param("changes".to_string(), query_value);
        }
        if let Some(ref s) = message {
            let query_value = s.to_string();
            req = req.with_query_param("message".to_string(), query_value);
        }
        if let Some(ref s) = reference {
            let query_value = s.to_string();
            req = req.with_query_param("reference".to_string(), query_value);
        }
        if let Some(ref s) = url {
            let query_value = s.to_string();
            req = req.with_query_param("url".to_string(), query_value);
        }
        if let Some(param_value) = content_type {
            req = req.with_header_param("Content-Type".to_string(), param_value.to_string());
        }
        req = req.with_body_param(upload);

        req.execute(self.configuration.borrow())
    }

    #[allow(unused_mut)]
    fn image_inspect_libpod(
        &self,
        name: &str,
    ) -> Pin<Box<dyn Future<Output = Result<models::ImageData, Error>> + Send>> {
        let mut req = __internal_request::Request::new(
            hyper::Method::GET,
            "/libpod/images/{name}/json".to_string(),
        );
        req = req.with_path_param("name".to_string(), name.to_string());

        req.execute(self.configuration.borrow())
    }

    #[allow(unused_mut)]
    fn image_list_libpod(
        &self,
        all: Option<bool>,
        filters: Option<&str>,
    ) -> Pin<Box<dyn Future<Output = Result<Vec<models::LibpodImageSummary>, Error>> + Send>> {
        let mut req =
            __internal_request::Request::new(hyper::Method::GET, "/libpod/images/json".to_string());
        if let Some(ref s) = all {
            let query_value = s.to_string();
            req = req.with_query_param("all".to_string(), query_value);
        }
        if let Some(ref s) = filters {
            let query_value = s.to_string();
            req = req.with_query_param("filters".to_string(), query_value);
        }

        req.execute(self.configuration.borrow())
    }

    #[allow(unused_mut)]
    fn image_load_libpod(
        &self,
        upload: &str,
    ) -> Pin<Box<dyn Future<Output = Result<models::ImageLoadReport, Error>> + Send>> {
        let mut req = __internal_request::Request::new(
            hyper::Method::POST,
            "/libpod/images/load".to_string(),
        );
        req = req.with_body_param(upload);

        req.execute(self.configuration.borrow())
    }

    #[allow(unused_mut)]
    fn image_prune_libpod(
        &self,
        all: Option<bool>,
        external: Option<bool>,
        filters: Option<&str>,
    ) -> Pin<Box<dyn Future<Output = Result<Vec<models::PruneReport>, Error>> + Send>> {
        let mut req = __internal_request::Request::new(
            hyper::Method::POST,
            "/libpod/images/prune".to_string(),
        );
        if let Some(ref s) = all {
            let query_value = s.to_string();
            req = req.with_query_param("all".to_string(), query_value);
        }
        if let Some(ref s) = external {
            let query_value = s.to_string();
            req = req.with_query_param("external".to_string(), query_value);
        }
        if let Some(ref s) = filters {
            let query_value = s.to_string();
            req = req.with_query_param("filters".to_string(), query_value);
        }

        req.execute(self.configuration.borrow())
    }

    #[allow(unused_mut)]
    fn image_pull_libpod(
        &self,
        reference: Option<&str>,
        quiet: Option<bool>,
        compat_mode: Option<bool>,
        arch: Option<&str>,
        os: Option<&str>,
        variant: Option<&str>,
        policy: Option<&str>,
        tls_verify: Option<bool>,
        all_tags: Option<bool>,
        x_registry_auth: Option<&str>,
    ) -> Pin<Box<dyn Future<Output = Result<models::LibpodImagesPullReport, Error>> + Send>> {
        let mut req = __internal_request::Request::new(
            hyper::Method::POST,
            "/libpod/images/pull".to_string(),
        );
        if let Some(ref s) = reference {
            let query_value = s.to_string();
            req = req.with_query_param("reference".to_string(), query_value);
        }
        if let Some(ref s) = quiet {
            let query_value = s.to_string();
            req = req.with_query_param("quiet".to_string(), query_value);
        }
        if let Some(ref s) = compat_mode {
            let query_value = s.to_string();
            req = req.with_query_param("compatMode".to_string(), query_value);
        }
        if let Some(ref s) = arch {
            let query_value = s.to_string();
            req = req.with_query_param("Arch".to_string(), query_value);
        }
        if let Some(ref s) = os {
            let query_value = s.to_string();
            req = req.with_query_param("OS".to_string(), query_value);
        }
        if let Some(ref s) = variant {
            let query_value = s.to_string();
            req = req.with_query_param("Variant".to_string(), query_value);
        }
        if let Some(ref s) = policy {
            let query_value = s.to_string();
            req = req.with_query_param("policy".to_string(), query_value);
        }
        if let Some(ref s) = tls_verify {
            let query_value = s.to_string();
            req = req.with_query_param("tlsVerify".to_string(), query_value);
        }
        if let Some(ref s) = all_tags {
            let query_value = s.to_string();
            req = req.with_query_param("allTags".to_string(), query_value);
        }
        if let Some(param_value) = x_registry_auth {
            req = req.with_header_param("X-Registry-Auth".to_string(), param_value.to_string());
        }

        req.execute(self.configuration.borrow())
    }

    #[allow(unused_mut)]
    fn image_push_libpod(
        &self,
        name: &str,
        destination: Option<&str>,
        force_compression_format: Option<bool>,
        tls_verify: Option<bool>,
        quiet: Option<bool>,
        x_registry_auth: Option<&str>,
    ) -> Pin<Box<dyn Future<Output = Result<std::path::PathBuf, Error>> + Send>> {
        let mut req = __internal_request::Request::new(
            hyper::Method::POST,
            "/libpod/images/{name}/push".to_string(),
        );
        if let Some(ref s) = destination {
            let query_value = s.to_string();
            req = req.with_query_param("destination".to_string(), query_value);
        }
        if let Some(ref s) = force_compression_format {
            let query_value = s.to_string();
            req = req.with_query_param("forceCompressionFormat".to_string(), query_value);
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
        if let Some(param_value) = x_registry_auth {
            req = req.with_header_param("X-Registry-Auth".to_string(), param_value.to_string());
        }

        req.execute(self.configuration.borrow())
    }

    #[allow(unused_mut)]
    fn image_resolve_libpod(
        &self,
        name: &str,
    ) -> Pin<Box<dyn Future<Output = Result<(), Error>> + Send>> {
        let mut req = __internal_request::Request::new(
            hyper::Method::GET,
            "/libpod/images/{name}/resolve".to_string(),
        );
        req = req.with_path_param("name".to_string(), name.to_string());
        req = req.returns_nothing();

        req.execute(self.configuration.borrow())
    }

    #[allow(unused_mut)]
    fn image_scp_libpod(
        &self,
        name: &str,
        destination: Option<&str>,
        quiet: Option<bool>,
    ) -> Pin<Box<dyn Future<Output = Result<models::ScpReport, Error>> + Send>> {
        let mut req = __internal_request::Request::new(
            hyper::Method::POST,
            "/libpod/images/scp/{name}".to_string(),
        );
        if let Some(ref s) = destination {
            let query_value = s.to_string();
            req = req.with_query_param("destination".to_string(), query_value);
        }
        if let Some(ref s) = quiet {
            let query_value = s.to_string();
            req = req.with_query_param("quiet".to_string(), query_value);
        }
        req = req.with_path_param("name".to_string(), name.to_string());

        req.execute(self.configuration.borrow())
    }

    #[allow(unused_mut)]
    fn image_search_libpod(
        &self,
        term: Option<&str>,
        limit: Option<i32>,
        filters: Option<&str>,
        tls_verify: Option<bool>,
        list_tags: Option<bool>,
    ) -> Pin<Box<dyn Future<Output = Result<models::ImageSearch200Response, Error>> + Send>> {
        let mut req = __internal_request::Request::new(
            hyper::Method::GET,
            "/libpod/images/search".to_string(),
        );
        if let Some(ref s) = term {
            let query_value = s.to_string();
            req = req.with_query_param("term".to_string(), query_value);
        }
        if let Some(ref s) = limit {
            let query_value = s.to_string();
            req = req.with_query_param("limit".to_string(), query_value);
        }
        if let Some(ref s) = filters {
            let query_value = s.to_string();
            req = req.with_query_param("filters".to_string(), query_value);
        }
        if let Some(ref s) = tls_verify {
            let query_value = s.to_string();
            req = req.with_query_param("tlsVerify".to_string(), query_value);
        }
        if let Some(ref s) = list_tags {
            let query_value = s.to_string();
            req = req.with_query_param("listTags".to_string(), query_value);
        }

        req.execute(self.configuration.borrow())
    }

    #[allow(unused_mut)]
    fn image_tag_libpod(
        &self,
        name: &str,
        repo: Option<&str>,
        tag: Option<&str>,
    ) -> Pin<Box<dyn Future<Output = Result<(), Error>> + Send>> {
        let mut req = __internal_request::Request::new(
            hyper::Method::POST,
            "/libpod/images/{name}/tag".to_string(),
        );
        if let Some(ref s) = repo {
            let query_value = s.to_string();
            req = req.with_query_param("repo".to_string(), query_value);
        }
        if let Some(ref s) = tag {
            let query_value = s.to_string();
            req = req.with_query_param("tag".to_string(), query_value);
        }
        req = req.with_path_param("name".to_string(), name.to_string());
        req = req.returns_nothing();

        req.execute(self.configuration.borrow())
    }

    #[allow(unused_mut)]
    fn image_tree_libpod(
        &self,
        name: &str,
        whatrequires: Option<bool>,
    ) -> Pin<Box<dyn Future<Output = Result<models::ImageTreeReport, Error>> + Send>> {
        let mut req = __internal_request::Request::new(
            hyper::Method::GET,
            "/libpod/images/{name}/tree".to_string(),
        );
        if let Some(ref s) = whatrequires {
            let query_value = s.to_string();
            req = req.with_query_param("whatrequires".to_string(), query_value);
        }
        req = req.with_path_param("name".to_string(), name.to_string());

        req.execute(self.configuration.borrow())
    }

    #[allow(unused_mut)]
    fn image_untag_libpod(
        &self,
        name: &str,
        repo: Option<&str>,
        tag: Option<&str>,
    ) -> Pin<Box<dyn Future<Output = Result<(), Error>> + Send>> {
        let mut req = __internal_request::Request::new(
            hyper::Method::POST,
            "/libpod/images/{name}/untag".to_string(),
        );
        if let Some(ref s) = repo {
            let query_value = s.to_string();
            req = req.with_query_param("repo".to_string(), query_value);
        }
        if let Some(ref s) = tag {
            let query_value = s.to_string();
            req = req.with_query_param("tag".to_string(), query_value);
        }
        req = req.with_path_param("name".to_string(), name.to_string());
        req = req.returns_nothing();

        req.execute(self.configuration.borrow())
    }
}

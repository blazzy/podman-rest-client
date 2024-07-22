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

pub struct PodsApiClient<C: Connect>
where
    C: Clone + std::marker::Send + Sync + 'static,
{
    configuration: Arc<configuration::Configuration<C>>,
}

impl<C: Connect> PodsApiClient<C>
where
    C: Clone + std::marker::Send + Sync,
{
    pub fn new(configuration: Arc<configuration::Configuration<C>>) -> PodsApiClient<C> {
        PodsApiClient { configuration }
    }
}

pub trait PodsApi: Send + Sync {
    fn generate_kube_libpod(
        &self,
        names: Vec<String>,
        service: Option<bool>,
        r#type: Option<&str>,
        replicas: Option<i32>,
        no_trunc: Option<bool>,
        podman_only: Option<bool>,
    ) -> Pin<Box<dyn Future<Output = Result<std::path::PathBuf, Error>> + Send>>;
    fn generate_systemd_libpod(
        &self,
        name: &str,
        use_name: Option<bool>,
        new: Option<bool>,
        no_header: Option<bool>,
        start_timeout: Option<i32>,
        stop_timeout: Option<i32>,
        restart_policy: Option<&str>,
        container_prefix: Option<&str>,
        pod_prefix: Option<&str>,
        separator: Option<&str>,
        restart_sec: Option<i32>,
        wants: Option<Vec<String>>,
        after: Option<Vec<String>>,
        requires: Option<Vec<String>>,
        additional_env_variables: Option<Vec<String>>,
    ) -> Pin<
        Box<dyn Future<Output = Result<std::collections::HashMap<String, String>, Error>> + Send>,
    >;
    fn kube_apply_libpod(
        &self,
        ca_cert_file: Option<&str>,
        kube_config: Option<&str>,
        namespace: Option<&str>,
        service: Option<bool>,
        file: Option<&str>,
        request: Option<&str>,
    ) -> Pin<Box<dyn Future<Output = Result<std::path::PathBuf, Error>> + Send>>;
    fn play_kube_down_libpod(
        &self,
        force: Option<bool>,
    ) -> Pin<Box<dyn Future<Output = Result<models::PlayKubeReport, Error>> + Send>>;
    fn play_kube_libpod(
        &self,
        annotations: Option<&str>,
        log_driver: Option<&str>,
        log_options: Option<Vec<String>>,
        network: Option<Vec<String>>,
        no_hosts: Option<bool>,
        no_trunc: Option<bool>,
        publish_ports: Option<Vec<String>>,
        publish_all_ports: Option<bool>,
        replace: Option<bool>,
        service_container: Option<bool>,
        start: Option<bool>,
        static_ips: Option<Vec<String>>,
        static_macs: Option<Vec<String>>,
        tls_verify: Option<bool>,
        userns: Option<&str>,
        wait: Option<bool>,
        request: Option<&str>,
    ) -> Pin<Box<dyn Future<Output = Result<models::PlayKubeReport, Error>> + Send>>;
    fn pod_create_libpod(
        &self,
        create: Option<models::PodSpecGenerator>,
    ) -> Pin<Box<dyn Future<Output = Result<models::IdResponse, Error>> + Send>>;
    fn pod_delete_libpod(
        &self,
        name: &str,
        force: Option<bool>,
    ) -> Pin<Box<dyn Future<Output = Result<models::PodRmReport, Error>> + Send>>;
    fn pod_exists_libpod(
        &self,
        name: &str,
    ) -> Pin<Box<dyn Future<Output = Result<(), Error>> + Send>>;
    fn pod_inspect_libpod(
        &self,
        name: &str,
    ) -> Pin<Box<dyn Future<Output = Result<models::InspectPodData, Error>> + Send>>;
    fn pod_kill_libpod(
        &self,
        name: &str,
        signal: Option<&str>,
    ) -> Pin<Box<dyn Future<Output = Result<models::PodKillReport, Error>> + Send>>;
    fn pod_list_libpod(
        &self,
        filters: Option<&str>,
    ) -> Pin<Box<dyn Future<Output = Result<Vec<models::ListPodsReport>, Error>> + Send>>;
    fn pod_pause_libpod(
        &self,
        name: &str,
    ) -> Pin<Box<dyn Future<Output = Result<models::PodPauseReport, Error>> + Send>>;
    fn pod_prune_libpod(
        &self,
    ) -> Pin<Box<dyn Future<Output = Result<models::PodPruneReport, Error>> + Send>>;
    fn pod_restart_libpod(
        &self,
        name: &str,
    ) -> Pin<Box<dyn Future<Output = Result<models::PodRestartReport, Error>> + Send>>;
    fn pod_start_libpod(
        &self,
        name: &str,
    ) -> Pin<Box<dyn Future<Output = Result<models::PodStartReport, Error>> + Send>>;
    fn pod_stats_all_libpod(
        &self,
        all: Option<bool>,
        names_or_ids: Option<Vec<String>>,
    ) -> Pin<Box<dyn Future<Output = Result<Vec<models::PodStatsReport>, Error>> + Send>>;
    fn pod_stop_libpod(
        &self,
        name: &str,
        t: Option<i32>,
    ) -> Pin<Box<dyn Future<Output = Result<models::PodStopReport, Error>> + Send>>;
    fn pod_top_libpod(
        &self,
        name: &str,
        stream: Option<bool>,
        delay: Option<i32>,
        ps_args: Option<&str>,
    ) -> Pin<Box<dyn Future<Output = Result<models::PodTopOkBody, Error>> + Send>>;
    fn pod_unpause_libpod(
        &self,
        name: &str,
    ) -> Pin<Box<dyn Future<Output = Result<models::PodUnpauseReport, Error>> + Send>>;
}

impl<C: Connect> PodsApi for PodsApiClient<C>
where
    C: Clone + std::marker::Send + Sync,
{
    #[allow(unused_mut)]
    fn generate_kube_libpod(
        &self,
        names: Vec<String>,
        service: Option<bool>,
        r#type: Option<&str>,
        replicas: Option<i32>,
        no_trunc: Option<bool>,
        podman_only: Option<bool>,
    ) -> Pin<Box<dyn Future<Output = Result<std::path::PathBuf, Error>> + Send>> {
        let mut req = __internal_request::Request::new(
            hyper::Method::GET,
            "/libpod/generate/kube".to_string(),
        );
        req = req.with_query_param("names".to_string(), names.join(",").to_string());
        if let Some(ref s) = service {
            let query_value = s.to_string();
            req = req.with_query_param("service".to_string(), query_value);
        }
        if let Some(ref s) = r#type {
            let query_value = s.to_string();
            req = req.with_query_param("type".to_string(), query_value);
        }
        if let Some(ref s) = replicas {
            let query_value = s.to_string();
            req = req.with_query_param("replicas".to_string(), query_value);
        }
        if let Some(ref s) = no_trunc {
            let query_value = s.to_string();
            req = req.with_query_param("noTrunc".to_string(), query_value);
        }
        if let Some(ref s) = podman_only {
            let query_value = s.to_string();
            req = req.with_query_param("podmanOnly".to_string(), query_value);
        }

        req.execute(self.configuration.borrow())
    }

    #[allow(unused_mut)]
    fn generate_systemd_libpod(
        &self,
        name: &str,
        use_name: Option<bool>,
        new: Option<bool>,
        no_header: Option<bool>,
        start_timeout: Option<i32>,
        stop_timeout: Option<i32>,
        restart_policy: Option<&str>,
        container_prefix: Option<&str>,
        pod_prefix: Option<&str>,
        separator: Option<&str>,
        restart_sec: Option<i32>,
        wants: Option<Vec<String>>,
        after: Option<Vec<String>>,
        requires: Option<Vec<String>>,
        additional_env_variables: Option<Vec<String>>,
    ) -> Pin<
        Box<dyn Future<Output = Result<std::collections::HashMap<String, String>, Error>> + Send>,
    > {
        let mut req = __internal_request::Request::new(
            hyper::Method::GET,
            "/libpod/generate/{name}/systemd".to_string(),
        );
        if let Some(ref s) = use_name {
            let query_value = s.to_string();
            req = req.with_query_param("useName".to_string(), query_value);
        }
        if let Some(ref s) = new {
            let query_value = s.to_string();
            req = req.with_query_param("new".to_string(), query_value);
        }
        if let Some(ref s) = no_header {
            let query_value = s.to_string();
            req = req.with_query_param("noHeader".to_string(), query_value);
        }
        if let Some(ref s) = start_timeout {
            let query_value = s.to_string();
            req = req.with_query_param("startTimeout".to_string(), query_value);
        }
        if let Some(ref s) = stop_timeout {
            let query_value = s.to_string();
            req = req.with_query_param("stopTimeout".to_string(), query_value);
        }
        if let Some(ref s) = restart_policy {
            let query_value = s.to_string();
            req = req.with_query_param("restartPolicy".to_string(), query_value);
        }
        if let Some(ref s) = container_prefix {
            let query_value = s.to_string();
            req = req.with_query_param("containerPrefix".to_string(), query_value);
        }
        if let Some(ref s) = pod_prefix {
            let query_value = s.to_string();
            req = req.with_query_param("podPrefix".to_string(), query_value);
        }
        if let Some(ref s) = separator {
            let query_value = s.to_string();
            req = req.with_query_param("separator".to_string(), query_value);
        }
        if let Some(ref s) = restart_sec {
            let query_value = s.to_string();
            req = req.with_query_param("restartSec".to_string(), query_value);
        }
        if let Some(ref s) = wants {
            let query_value = s
                .iter()
                .map(|s| s.to_string())
                .collect::<Vec<String>>()
                .join(",");
            req = req.with_query_param("wants".to_string(), query_value);
        }
        if let Some(ref s) = after {
            let query_value = s
                .iter()
                .map(|s| s.to_string())
                .collect::<Vec<String>>()
                .join(",");
            req = req.with_query_param("after".to_string(), query_value);
        }
        if let Some(ref s) = requires {
            let query_value = s
                .iter()
                .map(|s| s.to_string())
                .collect::<Vec<String>>()
                .join(",");
            req = req.with_query_param("requires".to_string(), query_value);
        }
        if let Some(ref s) = additional_env_variables {
            let query_value = s
                .iter()
                .map(|s| s.to_string())
                .collect::<Vec<String>>()
                .join(",");
            req = req.with_query_param("additionalEnvVariables".to_string(), query_value);
        }
        req = req.with_path_param("name".to_string(), name.to_string());

        req.execute(self.configuration.borrow())
    }

    #[allow(unused_mut)]
    fn kube_apply_libpod(
        &self,
        ca_cert_file: Option<&str>,
        kube_config: Option<&str>,
        namespace: Option<&str>,
        service: Option<bool>,
        file: Option<&str>,
        request: Option<&str>,
    ) -> Pin<Box<dyn Future<Output = Result<std::path::PathBuf, Error>> + Send>> {
        let mut req =
            __internal_request::Request::new(hyper::Method::POST, "/libpod/kube/apply".to_string());
        if let Some(ref s) = ca_cert_file {
            let query_value = s.to_string();
            req = req.with_query_param("caCertFile".to_string(), query_value);
        }
        if let Some(ref s) = kube_config {
            let query_value = s.to_string();
            req = req.with_query_param("kubeConfig".to_string(), query_value);
        }
        if let Some(ref s) = namespace {
            let query_value = s.to_string();
            req = req.with_query_param("namespace".to_string(), query_value);
        }
        if let Some(ref s) = service {
            let query_value = s.to_string();
            req = req.with_query_param("service".to_string(), query_value);
        }
        if let Some(ref s) = file {
            let query_value = s.to_string();
            req = req.with_query_param("file".to_string(), query_value);
        }
        req = req.with_body_param(request);

        req.execute(self.configuration.borrow())
    }

    #[allow(unused_mut)]
    fn play_kube_down_libpod(
        &self,
        force: Option<bool>,
    ) -> Pin<Box<dyn Future<Output = Result<models::PlayKubeReport, Error>> + Send>> {
        let mut req = __internal_request::Request::new(
            hyper::Method::DELETE,
            "/libpod/play/kube".to_string(),
        );
        if let Some(ref s) = force {
            let query_value = s.to_string();
            req = req.with_query_param("force".to_string(), query_value);
        }

        req.execute(self.configuration.borrow())
    }

    #[allow(unused_mut)]
    fn play_kube_libpod(
        &self,
        annotations: Option<&str>,
        log_driver: Option<&str>,
        log_options: Option<Vec<String>>,
        network: Option<Vec<String>>,
        no_hosts: Option<bool>,
        no_trunc: Option<bool>,
        publish_ports: Option<Vec<String>>,
        publish_all_ports: Option<bool>,
        replace: Option<bool>,
        service_container: Option<bool>,
        start: Option<bool>,
        static_ips: Option<Vec<String>>,
        static_macs: Option<Vec<String>>,
        tls_verify: Option<bool>,
        userns: Option<&str>,
        wait: Option<bool>,
        request: Option<&str>,
    ) -> Pin<Box<dyn Future<Output = Result<models::PlayKubeReport, Error>> + Send>> {
        let mut req =
            __internal_request::Request::new(hyper::Method::POST, "/libpod/play/kube".to_string());
        if let Some(ref s) = annotations {
            let query_value = s.to_string();
            req = req.with_query_param("annotations".to_string(), query_value);
        }
        if let Some(ref s) = log_driver {
            let query_value = s.to_string();
            req = req.with_query_param("logDriver".to_string(), query_value);
        }
        if let Some(ref s) = log_options {
            let query_value = s
                .iter()
                .map(|s| s.to_string())
                .collect::<Vec<String>>()
                .join(",");
            req = req.with_query_param("logOptions".to_string(), query_value);
        }
        if let Some(ref s) = network {
            let query_value = s
                .iter()
                .map(|s| s.to_string())
                .collect::<Vec<String>>()
                .join(",");
            req = req.with_query_param("network".to_string(), query_value);
        }
        if let Some(ref s) = no_hosts {
            let query_value = s.to_string();
            req = req.with_query_param("noHosts".to_string(), query_value);
        }
        if let Some(ref s) = no_trunc {
            let query_value = s.to_string();
            req = req.with_query_param("noTrunc".to_string(), query_value);
        }
        if let Some(ref s) = publish_ports {
            let query_value = s
                .iter()
                .map(|s| s.to_string())
                .collect::<Vec<String>>()
                .join(",");
            req = req.with_query_param("publishPorts".to_string(), query_value);
        }
        if let Some(ref s) = publish_all_ports {
            let query_value = s.to_string();
            req = req.with_query_param("publishAllPorts".to_string(), query_value);
        }
        if let Some(ref s) = replace {
            let query_value = s.to_string();
            req = req.with_query_param("replace".to_string(), query_value);
        }
        if let Some(ref s) = service_container {
            let query_value = s.to_string();
            req = req.with_query_param("serviceContainer".to_string(), query_value);
        }
        if let Some(ref s) = start {
            let query_value = s.to_string();
            req = req.with_query_param("start".to_string(), query_value);
        }
        if let Some(ref s) = static_ips {
            let query_value = s
                .iter()
                .map(|s| s.to_string())
                .collect::<Vec<String>>()
                .join(",");
            req = req.with_query_param("staticIPs".to_string(), query_value);
        }
        if let Some(ref s) = static_macs {
            let query_value = s
                .iter()
                .map(|s| s.to_string())
                .collect::<Vec<String>>()
                .join(",");
            req = req.with_query_param("staticMACs".to_string(), query_value);
        }
        if let Some(ref s) = tls_verify {
            let query_value = s.to_string();
            req = req.with_query_param("tlsVerify".to_string(), query_value);
        }
        if let Some(ref s) = userns {
            let query_value = s.to_string();
            req = req.with_query_param("userns".to_string(), query_value);
        }
        if let Some(ref s) = wait {
            let query_value = s.to_string();
            req = req.with_query_param("wait".to_string(), query_value);
        }
        req = req.with_body_param(request);

        req.execute(self.configuration.borrow())
    }

    #[allow(unused_mut)]
    fn pod_create_libpod(
        &self,
        create: Option<models::PodSpecGenerator>,
    ) -> Pin<Box<dyn Future<Output = Result<models::IdResponse, Error>> + Send>> {
        let mut req = __internal_request::Request::new(
            hyper::Method::POST,
            "/libpod/pods/create".to_string(),
        );
        req = req.with_body_param(create);

        req.execute(self.configuration.borrow())
    }

    #[allow(unused_mut)]
    fn pod_delete_libpod(
        &self,
        name: &str,
        force: Option<bool>,
    ) -> Pin<Box<dyn Future<Output = Result<models::PodRmReport, Error>> + Send>> {
        let mut req = __internal_request::Request::new(
            hyper::Method::DELETE,
            "/libpod/pods/{name}".to_string(),
        );
        if let Some(ref s) = force {
            let query_value = s.to_string();
            req = req.with_query_param("force".to_string(), query_value);
        }
        req = req.with_path_param("name".to_string(), name.to_string());

        req.execute(self.configuration.borrow())
    }

    #[allow(unused_mut)]
    fn pod_exists_libpod(
        &self,
        name: &str,
    ) -> Pin<Box<dyn Future<Output = Result<(), Error>> + Send>> {
        let mut req = __internal_request::Request::new(
            hyper::Method::GET,
            "/libpod/pods/{name}/exists".to_string(),
        );
        req = req.with_path_param("name".to_string(), name.to_string());
        req = req.returns_nothing();

        req.execute(self.configuration.borrow())
    }

    #[allow(unused_mut)]
    fn pod_inspect_libpod(
        &self,
        name: &str,
    ) -> Pin<Box<dyn Future<Output = Result<models::InspectPodData, Error>> + Send>> {
        let mut req = __internal_request::Request::new(
            hyper::Method::GET,
            "/libpod/pods/{name}/json".to_string(),
        );
        req = req.with_path_param("name".to_string(), name.to_string());

        req.execute(self.configuration.borrow())
    }

    #[allow(unused_mut)]
    fn pod_kill_libpod(
        &self,
        name: &str,
        signal: Option<&str>,
    ) -> Pin<Box<dyn Future<Output = Result<models::PodKillReport, Error>> + Send>> {
        let mut req = __internal_request::Request::new(
            hyper::Method::POST,
            "/libpod/pods/{name}/kill".to_string(),
        );
        if let Some(ref s) = signal {
            let query_value = s.to_string();
            req = req.with_query_param("signal".to_string(), query_value);
        }
        req = req.with_path_param("name".to_string(), name.to_string());

        req.execute(self.configuration.borrow())
    }

    #[allow(unused_mut)]
    fn pod_list_libpod(
        &self,
        filters: Option<&str>,
    ) -> Pin<Box<dyn Future<Output = Result<Vec<models::ListPodsReport>, Error>> + Send>> {
        let mut req =
            __internal_request::Request::new(hyper::Method::GET, "/libpod/pods/json".to_string());
        if let Some(ref s) = filters {
            let query_value = s.to_string();
            req = req.with_query_param("filters".to_string(), query_value);
        }

        req.execute(self.configuration.borrow())
    }

    #[allow(unused_mut)]
    fn pod_pause_libpod(
        &self,
        name: &str,
    ) -> Pin<Box<dyn Future<Output = Result<models::PodPauseReport, Error>> + Send>> {
        let mut req = __internal_request::Request::new(
            hyper::Method::POST,
            "/libpod/pods/{name}/pause".to_string(),
        );
        req = req.with_path_param("name".to_string(), name.to_string());

        req.execute(self.configuration.borrow())
    }

    #[allow(unused_mut)]
    fn pod_prune_libpod(
        &self,
    ) -> Pin<Box<dyn Future<Output = Result<models::PodPruneReport, Error>> + Send>> {
        let mut req =
            __internal_request::Request::new(hyper::Method::POST, "/libpod/pods/prune".to_string());

        req.execute(self.configuration.borrow())
    }

    #[allow(unused_mut)]
    fn pod_restart_libpod(
        &self,
        name: &str,
    ) -> Pin<Box<dyn Future<Output = Result<models::PodRestartReport, Error>> + Send>> {
        let mut req = __internal_request::Request::new(
            hyper::Method::POST,
            "/libpod/pods/{name}/restart".to_string(),
        );
        req = req.with_path_param("name".to_string(), name.to_string());

        req.execute(self.configuration.borrow())
    }

    #[allow(unused_mut)]
    fn pod_start_libpod(
        &self,
        name: &str,
    ) -> Pin<Box<dyn Future<Output = Result<models::PodStartReport, Error>> + Send>> {
        let mut req = __internal_request::Request::new(
            hyper::Method::POST,
            "/libpod/pods/{name}/start".to_string(),
        );
        req = req.with_path_param("name".to_string(), name.to_string());

        req.execute(self.configuration.borrow())
    }

    #[allow(unused_mut)]
    fn pod_stats_all_libpod(
        &self,
        all: Option<bool>,
        names_or_ids: Option<Vec<String>>,
    ) -> Pin<Box<dyn Future<Output = Result<Vec<models::PodStatsReport>, Error>> + Send>> {
        let mut req =
            __internal_request::Request::new(hyper::Method::GET, "/libpod/pods/stats".to_string());
        if let Some(ref s) = all {
            let query_value = s.to_string();
            req = req.with_query_param("all".to_string(), query_value);
        }
        if let Some(ref s) = names_or_ids {
            let query_value = s
                .iter()
                .map(|s| s.to_string())
                .collect::<Vec<String>>()
                .join(",");
            req = req.with_query_param("namesOrIDs".to_string(), query_value);
        }

        req.execute(self.configuration.borrow())
    }

    #[allow(unused_mut)]
    fn pod_stop_libpod(
        &self,
        name: &str,
        t: Option<i32>,
    ) -> Pin<Box<dyn Future<Output = Result<models::PodStopReport, Error>> + Send>> {
        let mut req = __internal_request::Request::new(
            hyper::Method::POST,
            "/libpod/pods/{name}/stop".to_string(),
        );
        if let Some(ref s) = t {
            let query_value = s.to_string();
            req = req.with_query_param("t".to_string(), query_value);
        }
        req = req.with_path_param("name".to_string(), name.to_string());

        req.execute(self.configuration.borrow())
    }

    #[allow(unused_mut)]
    fn pod_top_libpod(
        &self,
        name: &str,
        stream: Option<bool>,
        delay: Option<i32>,
        ps_args: Option<&str>,
    ) -> Pin<Box<dyn Future<Output = Result<models::PodTopOkBody, Error>> + Send>> {
        let mut req = __internal_request::Request::new(
            hyper::Method::GET,
            "/libpod/pods/{name}/top".to_string(),
        );
        if let Some(ref s) = stream {
            let query_value = s.to_string();
            req = req.with_query_param("stream".to_string(), query_value);
        }
        if let Some(ref s) = delay {
            let query_value = s.to_string();
            req = req.with_query_param("delay".to_string(), query_value);
        }
        if let Some(ref s) = ps_args {
            let query_value = s.to_string();
            req = req.with_query_param("ps_args".to_string(), query_value);
        }
        req = req.with_path_param("name".to_string(), name.to_string());

        req.execute(self.configuration.borrow())
    }

    #[allow(unused_mut)]
    fn pod_unpause_libpod(
        &self,
        name: &str,
    ) -> Pin<Box<dyn Future<Output = Result<models::PodUnpauseReport, Error>> + Send>> {
        let mut req = __internal_request::Request::new(
            hyper::Method::POST,
            "/libpod/pods/{name}/unpause".to_string(),
        );
        req = req.with_path_param("name".to_string(), name.to_string());

        req.execute(self.configuration.borrow())
    }
}

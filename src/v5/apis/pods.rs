use crate::api_common::config::HasConfig;
use crate::api_common::request;
use crate::api_common::Error;
use std::future::Future;
use std::pin::Pin;
pub trait Pods: HasConfig + Send + Sync {
    /// DELETE /libpod/pods/{name}
    /// Remove pod
    fn pod_delete_libpod<'a>(
        &'a self,
        name: &'a str,
        params: Option<super::super::params::PodDeleteLibpod>,
    ) -> Pin<Box<dyn Future<Output = Result<super::super::models::PodRmReport, Error>> + Send + 'a>>
    {
        Box::pin(request::execute_request_json(
            self.get_config(),
            (|| {
                let mut request_url = url::Url::parse(self.get_config().get_base_path())?;
                let mut request_path = request_url.path().to_owned();
                if request_path.ends_with('/') {
                    request_path.pop();
                }
                request_path.push_str("/libpod/pods/{name}");
                request_path = request_path.replace("{name}", name);
                request_url.set_path(&request_path);
                let mut req_builder = self.get_config().req_builder("DELETE")?;
                if let Some(params) = params {
                    let mut query_pairs = request_url.query_pairs_mut();
                    if let Some(force) = params.force {
                        query_pairs.append_pair("force", &force.to_string());
                    }
                }
                let hyper_uri: hyper::Uri = request_url.as_str().parse()?;
                req_builder = req_builder.uri(hyper_uri);
                Ok(req_builder.body(String::new())?)
            })(),
        ))
    }
    /// GET /libpod/pods/{name}/exists
    /// Pod exists
    /// Check if a pod exists by name or ID
    fn pod_exists_libpod<'a>(
        &'a self,
        name: &'a str,
    ) -> Pin<Box<dyn Future<Output = Result<(), Error>> + Send + 'a>> {
        Box::pin(request::execute_request_unit(
            self.get_config(),
            (|| {
                let mut request_url = url::Url::parse(self.get_config().get_base_path())?;
                let mut request_path = request_url.path().to_owned();
                if request_path.ends_with('/') {
                    request_path.pop();
                }
                request_path.push_str("/libpod/pods/{name}/exists");
                request_path = request_path.replace("{name}", name);
                request_url.set_path(&request_path);
                let mut req_builder = self.get_config().req_builder("GET")?;
                let hyper_uri: hyper::Uri = request_url.as_str().parse()?;
                req_builder = req_builder.uri(hyper_uri);
                Ok(req_builder.body(String::new())?)
            })(),
        ))
    }
    /// GET /libpod/pods/{name}/json
    /// Inspect pod
    fn pod_inspect_libpod<'a>(
        &'a self,
        name: &'a str,
    ) -> Pin<
        Box<dyn Future<Output = Result<super::super::models::InspectPodData, Error>> + Send + 'a>,
    > {
        Box::pin(request::execute_request_json(
            self.get_config(),
            (|| {
                let mut request_url = url::Url::parse(self.get_config().get_base_path())?;
                let mut request_path = request_url.path().to_owned();
                if request_path.ends_with('/') {
                    request_path.pop();
                }
                request_path.push_str("/libpod/pods/{name}/json");
                request_path = request_path.replace("{name}", name);
                request_url.set_path(&request_path);
                let mut req_builder = self.get_config().req_builder("GET")?;
                let hyper_uri: hyper::Uri = request_url.as_str().parse()?;
                req_builder = req_builder.uri(hyper_uri);
                Ok(req_builder.body(String::new())?)
            })(),
        ))
    }
    /// POST /libpod/pods/{name}/kill
    /// Kill a pod
    fn pod_kill_libpod<'a>(
        &'a self,
        name: &'a str,
        params: Option<super::super::params::PodKillLibpod<'a>>,
    ) -> Pin<Box<dyn Future<Output = Result<super::super::models::PodKillReport, Error>> + Send + 'a>>
    {
        Box::pin(request::execute_request_json(
            self.get_config(),
            (|| {
                let mut request_url = url::Url::parse(self.get_config().get_base_path())?;
                let mut request_path = request_url.path().to_owned();
                if request_path.ends_with('/') {
                    request_path.pop();
                }
                request_path.push_str("/libpod/pods/{name}/kill");
                request_path = request_path.replace("{name}", name);
                request_url.set_path(&request_path);
                let mut req_builder = self.get_config().req_builder("POST")?;
                if let Some(params) = params {
                    let mut query_pairs = request_url.query_pairs_mut();
                    if let Some(signal) = params.signal {
                        query_pairs.append_pair("signal", signal);
                    }
                }
                let hyper_uri: hyper::Uri = request_url.as_str().parse()?;
                req_builder = req_builder.uri(hyper_uri);
                Ok(req_builder.body(String::new())?)
            })(),
        ))
    }
    /// POST /libpod/pods/{name}/pause
    /// Pause a pod
    /// Pause a pod
    fn pod_pause_libpod<'a>(
        &'a self,
        name: &'a str,
    ) -> Pin<
        Box<dyn Future<Output = Result<super::super::models::PodPauseReport, Error>> + Send + 'a>,
    > {
        Box::pin(request::execute_request_json(
            self.get_config(),
            (|| {
                let mut request_url = url::Url::parse(self.get_config().get_base_path())?;
                let mut request_path = request_url.path().to_owned();
                if request_path.ends_with('/') {
                    request_path.pop();
                }
                request_path.push_str("/libpod/pods/{name}/pause");
                request_path = request_path.replace("{name}", name);
                request_url.set_path(&request_path);
                let mut req_builder = self.get_config().req_builder("POST")?;
                let hyper_uri: hyper::Uri = request_url.as_str().parse()?;
                req_builder = req_builder.uri(hyper_uri);
                Ok(req_builder.body(String::new())?)
            })(),
        ))
    }
    /// POST /libpod/pods/{name}/restart
    /// Restart a pod
    fn pod_restart_libpod<'a>(
        &'a self,
        name: &'a str,
    ) -> Pin<
        Box<dyn Future<Output = Result<super::super::models::PodRestartReport, Error>> + Send + 'a>,
    > {
        Box::pin(request::execute_request_json(
            self.get_config(),
            (|| {
                let mut request_url = url::Url::parse(self.get_config().get_base_path())?;
                let mut request_path = request_url.path().to_owned();
                if request_path.ends_with('/') {
                    request_path.pop();
                }
                request_path.push_str("/libpod/pods/{name}/restart");
                request_path = request_path.replace("{name}", name);
                request_url.set_path(&request_path);
                let mut req_builder = self.get_config().req_builder("POST")?;
                let hyper_uri: hyper::Uri = request_url.as_str().parse()?;
                req_builder = req_builder.uri(hyper_uri);
                Ok(req_builder.body(String::new())?)
            })(),
        ))
    }
    /// POST /libpod/pods/{name}/start
    /// Start a pod
    fn pod_start_libpod<'a>(
        &'a self,
        name: &'a str,
    ) -> Pin<
        Box<dyn Future<Output = Result<super::super::models::PodStartReport, Error>> + Send + 'a>,
    > {
        Box::pin(request::execute_request_json(
            self.get_config(),
            (|| {
                let mut request_url = url::Url::parse(self.get_config().get_base_path())?;
                let mut request_path = request_url.path().to_owned();
                if request_path.ends_with('/') {
                    request_path.pop();
                }
                request_path.push_str("/libpod/pods/{name}/start");
                request_path = request_path.replace("{name}", name);
                request_url.set_path(&request_path);
                let mut req_builder = self.get_config().req_builder("POST")?;
                let hyper_uri: hyper::Uri = request_url.as_str().parse()?;
                req_builder = req_builder.uri(hyper_uri);
                Ok(req_builder.body(String::new())?)
            })(),
        ))
    }
    /// POST /libpod/pods/{name}/stop
    /// Stop a pod
    fn pod_stop_libpod<'a>(
        &'a self,
        name: &'a str,
        params: Option<super::super::params::PodStopLibpod>,
    ) -> Pin<Box<dyn Future<Output = Result<super::super::models::PodStopReport, Error>> + Send + 'a>>
    {
        Box::pin(request::execute_request_json(
            self.get_config(),
            (|| {
                let mut request_url = url::Url::parse(self.get_config().get_base_path())?;
                let mut request_path = request_url.path().to_owned();
                if request_path.ends_with('/') {
                    request_path.pop();
                }
                request_path.push_str("/libpod/pods/{name}/stop");
                request_path = request_path.replace("{name}", name);
                request_url.set_path(&request_path);
                let mut req_builder = self.get_config().req_builder("POST")?;
                if let Some(params) = params {
                    let mut query_pairs = request_url.query_pairs_mut();
                    if let Some(t) = params.t {
                        query_pairs.append_pair("t", &t.to_string());
                    }
                }
                let hyper_uri: hyper::Uri = request_url.as_str().parse()?;
                req_builder = req_builder.uri(hyper_uri);
                Ok(req_builder.body(String::new())?)
            })(),
        ))
    }
    /// GET /libpod/pods/{name}/top
    /// List processes
    /// List processes running inside a pod
    fn pod_top_libpod<'a>(
        &'a self,
        name: &'a str,
        params: Option<super::super::params::PodTopLibpod<'a>>,
    ) -> Pin<Box<dyn Future<Output = Result<super::super::models::PodTopOkBody, Error>> + Send + 'a>>
    {
        Box::pin(request::execute_request_json(
            self.get_config(),
            (|| {
                let mut request_url = url::Url::parse(self.get_config().get_base_path())?;
                let mut request_path = request_url.path().to_owned();
                if request_path.ends_with('/') {
                    request_path.pop();
                }
                request_path.push_str("/libpod/pods/{name}/top");
                request_path = request_path.replace("{name}", name);
                request_url.set_path(&request_path);
                let mut req_builder = self.get_config().req_builder("GET")?;
                if let Some(params) = params {
                    let mut query_pairs = request_url.query_pairs_mut();
                    if let Some(stream) = params.stream {
                        query_pairs.append_pair("stream", &stream.to_string());
                    }
                    if let Some(delay) = params.delay {
                        query_pairs.append_pair("delay", &delay.to_string());
                    }
                    if let Some(ps_args) = params.ps_args {
                        query_pairs.append_pair("ps_args", ps_args);
                    }
                }
                let hyper_uri: hyper::Uri = request_url.as_str().parse()?;
                req_builder = req_builder.uri(hyper_uri);
                Ok(req_builder.body(String::new())?)
            })(),
        ))
    }
    /// POST /libpod/pods/{name}/unpause
    /// Unpause a pod
    fn pod_unpause_libpod<'a>(
        &'a self,
        name: &'a str,
    ) -> Pin<
        Box<dyn Future<Output = Result<super::super::models::PodUnpauseReport, Error>> + Send + 'a>,
    > {
        Box::pin(request::execute_request_json(
            self.get_config(),
            (|| {
                let mut request_url = url::Url::parse(self.get_config().get_base_path())?;
                let mut request_path = request_url.path().to_owned();
                if request_path.ends_with('/') {
                    request_path.pop();
                }
                request_path.push_str("/libpod/pods/{name}/unpause");
                request_path = request_path.replace("{name}", name);
                request_url.set_path(&request_path);
                let mut req_builder = self.get_config().req_builder("POST")?;
                let hyper_uri: hyper::Uri = request_url.as_str().parse()?;
                req_builder = req_builder.uri(hyper_uri);
                Ok(req_builder.body(String::new())?)
            })(),
        ))
    }
    /// POST /libpod/pods/create
    /// Create a pod
    fn pod_create_libpod<'a>(
        &'a self,
        create: super::super::models::PodSpecGenerator,
    ) -> Pin<Box<dyn Future<Output = Result<super::super::models::IdResponse, Error>> + Send + 'a>>
    {
        Box::pin(request::execute_request_json(
            self.get_config(),
            (|| {
                let mut request_url = url::Url::parse(self.get_config().get_base_path())?;
                let mut request_path = request_url.path().to_owned();
                if request_path.ends_with('/') {
                    request_path.pop();
                }
                request_path.push_str("/libpod/pods/create");
                request_url.set_path(&request_path);
                let mut req_builder = self.get_config().req_builder("POST")?;
                let hyper_uri: hyper::Uri = request_url.as_str().parse()?;
                req_builder = req_builder.uri(hyper_uri);
                let body = serde_json::to_string(&create)?;
                req_builder = req_builder.header(hyper::header::CONTENT_TYPE, "application/json");
                req_builder = req_builder.header(hyper::header::CONTENT_LENGTH, body.len());
                Ok(req_builder.body(body)?)
            })(),
        ))
    }
    /// GET /libpod/pods/json
    /// List pods
    fn pod_list_libpod<'a>(
        &'a self,
        params: Option<super::super::params::PodListLibpod<'a>>,
    ) -> Pin<
        Box<
            dyn Future<Output = Result<Vec<super::super::models::ListPodsReport>, Error>>
                + Send
                + 'a,
        >,
    > {
        Box::pin(request::execute_request_json(
            self.get_config(),
            (|| {
                let mut request_url = url::Url::parse(self.get_config().get_base_path())?;
                let mut request_path = request_url.path().to_owned();
                if request_path.ends_with('/') {
                    request_path.pop();
                }
                request_path.push_str("/libpod/pods/json");
                request_url.set_path(&request_path);
                let mut req_builder = self.get_config().req_builder("GET")?;
                if let Some(params) = params {
                    let mut query_pairs = request_url.query_pairs_mut();
                    if let Some(filters) = params.filters {
                        query_pairs.append_pair("filters", filters);
                    }
                }
                let hyper_uri: hyper::Uri = request_url.as_str().parse()?;
                req_builder = req_builder.uri(hyper_uri);
                Ok(req_builder.body(String::new())?)
            })(),
        ))
    }
    /// POST /libpod/pods/prune
    /// Prune unused pods
    fn pod_prune_libpod<'a>(
        &'a self,
    ) -> Pin<
        Box<dyn Future<Output = Result<super::super::models::PodPruneReport, Error>> + Send + 'a>,
    > {
        Box::pin(request::execute_request_json(
            self.get_config(),
            (|| {
                let mut request_url = url::Url::parse(self.get_config().get_base_path())?;
                let mut request_path = request_url.path().to_owned();
                if request_path.ends_with('/') {
                    request_path.pop();
                }
                request_path.push_str("/libpod/pods/prune");
                request_url.set_path(&request_path);
                let mut req_builder = self.get_config().req_builder("POST")?;
                let hyper_uri: hyper::Uri = request_url.as_str().parse()?;
                req_builder = req_builder.uri(hyper_uri);
                Ok(req_builder.body(String::new())?)
            })(),
        ))
    }
    /// GET /libpod/pods/stats
    /// Statistics for one or more pods
    /// Display a live stream of resource usage statistics for the containers in one or more pods
    fn pod_stats_all_libpod<'a>(
        &'a self,
        params: Option<super::super::params::PodStatsAllLibpod<'a>>,
    ) -> Pin<
        Box<
            dyn Future<Output = Result<Vec<super::super::models::PodStatsReport>, Error>>
                + Send
                + 'a,
        >,
    > {
        Box::pin(request::execute_request_json(
            self.get_config(),
            (|| {
                let mut request_url = url::Url::parse(self.get_config().get_base_path())?;
                let mut request_path = request_url.path().to_owned();
                if request_path.ends_with('/') {
                    request_path.pop();
                }
                request_path.push_str("/libpod/pods/stats");
                request_url.set_path(&request_path);
                let mut req_builder = self.get_config().req_builder("GET")?;
                if let Some(params) = params {
                    let mut query_pairs = request_url.query_pairs_mut();
                    if let Some(all) = params.all {
                        query_pairs.append_pair("all", &all.to_string());
                    }
                    if let Some(names_or_i_ds) = params.names_or_i_ds {
                        for value in names_or_i_ds {
                            query_pairs.append_pair("namesOrIDs", &value.to_string());
                        }
                    }
                }
                let hyper_uri: hyper::Uri = request_url.as_str().parse()?;
                req_builder = req_builder.uri(hyper_uri);
                Ok(req_builder.body(String::new())?)
            })(),
        ))
    }
}

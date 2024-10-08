use crate::api_common::config::HasConfig;
use crate::api_common::request;
use crate::api_common::Error;
use http::request::Builder;
use std::future::Future;
use std::pin::Pin;
pub trait System: HasConfig + Send + Sync {
    /// GET /libpod/events
    ///
    /// Get events
    ///
    /// Returns events filtered on query parameters
    fn system_events_libpod<'a>(
        &'a self,
        params: Option<crate::v5::params::SystemEventsLibpod<'a>>,
    ) -> Pin<Box<dyn Future<Output = Result<(), Error>> + Send + 'a>> {
        Box::pin(request::execute_request_unit(
            self.get_config(),
            move |mut req_builder: Builder| {
                req_builder = req_builder.method("GET");
                let mut request_url = url::Url::parse(self.get_config().get_base_path())?;
                let mut request_path = request_url.path().to_owned();
                if request_path.ends_with('/') {
                    request_path.pop();
                }
                request_path.push_str("/libpod/events");
                request_url.set_path(&request_path);
                if let Some(params) = &params {
                    let mut query_pairs = request_url.query_pairs_mut();
                    if let Some(since) = params.since {
                        query_pairs.append_pair("since", since);
                    }
                    if let Some(until) = params.until {
                        query_pairs.append_pair("until", until);
                    }
                    if let Some(filters) = params.filters {
                        query_pairs.append_pair("filters", filters);
                    }
                    if let Some(stream) = params.stream {
                        query_pairs.append_pair("stream", &stream.to_string());
                    }
                }
                let hyper_uri: hyper::Uri = request_url.as_str().parse()?;
                req_builder = req_builder.uri(hyper_uri);
                Ok(req_builder.body(String::new())?)
            },
        ))
    }
    /// GET /libpod/info
    ///
    /// Get info
    ///
    /// Returns information on the system and libpod configuration
    fn system_info_libpod<'a>(
        &'a self,
    ) -> Pin<Box<dyn Future<Output = Result<crate::v5::models::LibpodInfo, Error>> + Send + 'a>>
    {
        Box::pin(request::execute_request_json(
            self.get_config(),
            move |mut req_builder: Builder| {
                req_builder = req_builder.method("GET");
                let mut request_url = url::Url::parse(self.get_config().get_base_path())?;
                let mut request_path = request_url.path().to_owned();
                if request_path.ends_with('/') {
                    request_path.pop();
                }
                request_path.push_str("/libpod/info");
                request_url.set_path(&request_path);
                let hyper_uri: hyper::Uri = request_url.as_str().parse()?;
                req_builder = req_builder.uri(hyper_uri);
                Ok(req_builder.body(String::new())?)
            },
        ))
    }
    /// GET /libpod/system/df
    ///
    /// Show disk usage
    ///
    /// Return information about disk usage for containers, images, and volumes
    fn system_data_usage_libpod<'a>(
        &'a self,
    ) -> Pin<Box<dyn Future<Output = Result<crate::v5::models::SystemDfReport, Error>> + Send + 'a>>
    {
        Box::pin(request::execute_request_json(
            self.get_config(),
            move |mut req_builder: Builder| {
                req_builder = req_builder.method("GET");
                let mut request_url = url::Url::parse(self.get_config().get_base_path())?;
                let mut request_path = request_url.path().to_owned();
                if request_path.ends_with('/') {
                    request_path.pop();
                }
                request_path.push_str("/libpod/system/df");
                request_url.set_path(&request_path);
                let hyper_uri: hyper::Uri = request_url.as_str().parse()?;
                req_builder = req_builder.uri(hyper_uri);
                Ok(req_builder.body(String::new())?)
            },
        ))
    }
    /// POST /libpod/system/prune
    ///
    /// Prune unused data
    fn system_prune_libpod<'a>(
        &'a self,
    ) -> Pin<
        Box<dyn Future<Output = Result<crate::v5::models::SystemPruneReport, Error>> + Send + 'a>,
    > {
        Box::pin(request::execute_request_json(
            self.get_config(),
            move |mut req_builder: Builder| {
                req_builder = req_builder.method("POST");
                let mut request_url = url::Url::parse(self.get_config().get_base_path())?;
                let mut request_path = request_url.path().to_owned();
                if request_path.ends_with('/') {
                    request_path.pop();
                }
                request_path.push_str("/libpod/system/prune");
                request_url.set_path(&request_path);
                let hyper_uri: hyper::Uri = request_url.as_str().parse()?;
                req_builder = req_builder.uri(hyper_uri);
                Ok(req_builder.body(String::new())?)
            },
        ))
    }
    /// GET /libpod/version
    ///
    /// Component Version information
    fn system_version_libpod<'a>(
        &'a self,
    ) -> Pin<
        Box<
            dyn Future<Output = Result<crate::v5::models::SystemComponentVersion, Error>>
                + Send
                + 'a,
        >,
    > {
        Box::pin(request::execute_request_json(
            self.get_config(),
            move |mut req_builder: Builder| {
                req_builder = req_builder.method("GET");
                let mut request_url = url::Url::parse(self.get_config().get_base_path())?;
                let mut request_path = request_url.path().to_owned();
                if request_path.ends_with('/') {
                    request_path.pop();
                }
                request_path.push_str("/libpod/version");
                request_url.set_path(&request_path);
                let hyper_uri: hyper::Uri = request_url.as_str().parse()?;
                req_builder = req_builder.uri(hyper_uri);
                Ok(req_builder.body(String::new())?)
            },
        ))
    }
}

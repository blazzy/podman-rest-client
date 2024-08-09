use super::super::config::HasConfig;
use super::super::request;
use super::super::Error;
#[async_trait::async_trait]
pub trait System: HasConfig + Send + Sync {
    /// GET /libpod/events
    /// Get events
    /// Returns events filtered on query parameters
    async fn system_events_libpod<'a>(
        &self,
        params: Option<super::super::params::SystemEventsLibpod<'a>>,
    ) -> Result<(), Error> {
        let mut request_url = url::Url::parse(self.get_config().get_base_path())?;
        let mut request_path = request_url.path().to_owned();
        if request_path.ends_with('/') {
            request_path.pop();
        }
        request_path.push_str("/libpod/events");
        request_url.set_path(&request_path);
        let mut req_builder = self.get_config().req_builder("GET")?;
        if let Some(params) = params {
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
        let request = req_builder.body(String::new())?;
        request::execute_request_unit(self.get_config(), request).await
    }
    /// GET /libpod/info
    /// Get info
    /// Returns information on the system and libpod configuration
    async fn system_info_libpod(&self) -> Result<super::super::models::LibpodInfo, Error> {
        let mut request_url = url::Url::parse(self.get_config().get_base_path())?;
        let mut request_path = request_url.path().to_owned();
        if request_path.ends_with('/') {
            request_path.pop();
        }
        request_path.push_str("/libpod/info");
        request_url.set_path(&request_path);
        let mut req_builder = self.get_config().req_builder("GET")?;
        let hyper_uri: hyper::Uri = request_url.as_str().parse()?;
        req_builder = req_builder.uri(hyper_uri);
        let request = req_builder.body(String::new())?;
        request::execute_request_json(self.get_config(), request).await
    }
    /// GET /libpod/system/df
    /// Show disk usage
    /// Return information about disk usage for containers, images, and volumes
    async fn system_data_usage_libpod(
        &self,
    ) -> Result<super::super::models::SystemDfReport, Error> {
        let mut request_url = url::Url::parse(self.get_config().get_base_path())?;
        let mut request_path = request_url.path().to_owned();
        if request_path.ends_with('/') {
            request_path.pop();
        }
        request_path.push_str("/libpod/system/df");
        request_url.set_path(&request_path);
        let mut req_builder = self.get_config().req_builder("GET")?;
        let hyper_uri: hyper::Uri = request_url.as_str().parse()?;
        req_builder = req_builder.uri(hyper_uri);
        let request = req_builder.body(String::new())?;
        request::execute_request_json(self.get_config(), request).await
    }
    /// POST /libpod/system/prune
    /// Prune unused data
    async fn system_prune_libpod(&self) -> Result<super::super::models::SystemPruneReport, Error> {
        let mut request_url = url::Url::parse(self.get_config().get_base_path())?;
        let mut request_path = request_url.path().to_owned();
        if request_path.ends_with('/') {
            request_path.pop();
        }
        request_path.push_str("/libpod/system/prune");
        request_url.set_path(&request_path);
        let mut req_builder = self.get_config().req_builder("POST")?;
        let hyper_uri: hyper::Uri = request_url.as_str().parse()?;
        req_builder = req_builder.uri(hyper_uri);
        let request = req_builder.body(String::new())?;
        request::execute_request_json(self.get_config(), request).await
    }
    /// GET /libpod/version
    /// Component Version information
    async fn system_version_libpod(
        &self,
    ) -> Result<super::super::models::SystemComponentVersion, Error> {
        let mut request_url = url::Url::parse(self.get_config().get_base_path())?;
        let mut request_path = request_url.path().to_owned();
        if request_path.ends_with('/') {
            request_path.pop();
        }
        request_path.push_str("/libpod/version");
        request_url.set_path(&request_path);
        let mut req_builder = self.get_config().req_builder("GET")?;
        let hyper_uri: hyper::Uri = request_url.as_str().parse()?;
        req_builder = req_builder.uri(hyper_uri);
        let request = req_builder.body(String::new())?;
        request::execute_request_json(self.get_config(), request).await
    }
}

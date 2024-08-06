use std::sync::Arc;

use super::super::config::ClientConfig;
use super::super::request;
use super::super::Error;

/// Actions related to networks for the compatibility endpoints
pub struct NetworksCompat {
    config: Arc<dyn ClientConfig>,
}

impl NetworksCompat {
    pub fn new(config: Arc<dyn ClientConfig>) -> NetworksCompat {
        NetworksCompat { config }
    }

    /// GET /networks
    /// List networks
    /// Display summary of network configurations
    pub async fn network_list<'a>(
        &self,
        params: Option<super::super::params::NetworkList<'a>>,
    ) -> Result<Vec<super::super::models::NetworkResource>, Error> {
        let mut request_url = url::Url::parse(self.config.get_base_path())?;

        let mut request_path = request_url.path().to_owned();
        if request_path.ends_with('/') {
            request_path.pop();
        }
        request_path.push_str("/networks");

        request_url.set_path(&request_path);

        let mut req_builder = self.config.req_builder("GET")?;

        if let Some(params) = params {
            let mut query_pairs = request_url.query_pairs_mut();
            if let Some(filters) = params.filters {
                query_pairs.append_pair("filters", &filters);
            }
        }

        let hyper_uri: hyper::Uri = request_url.as_str().parse()?;
        req_builder = req_builder.uri(hyper_uri);
        let request = req_builder.body(String::new())?;
        request::execute_request_json(&*self.config, request).await
    }

    /// DELETE /networks/{name}
    /// Remove a network
    /// Remove a network
    pub async fn network_delete(&self, name: &str) -> Result<(), Error> {
        let mut request_url = url::Url::parse(self.config.get_base_path())?;

        let mut request_path = request_url.path().to_owned();
        if request_path.ends_with('/') {
            request_path.pop();
        }
        request_path.push_str("/networks/{name}");
        request_path = request_path.replace("{name}", name);

        request_url.set_path(&request_path);

        let mut req_builder = self.config.req_builder("DELETE")?;

        let hyper_uri: hyper::Uri = request_url.as_str().parse()?;
        req_builder = req_builder.uri(hyper_uri);
        let request = req_builder.body(String::new())?;
        request::execute_request_unit(&*self.config, request).await
    }

    /// GET /networks/{name}
    /// Inspect a network
    /// Display low level configuration network
    pub async fn network_inspect<'a>(
        &self,
        name: &str,
        params: Option<super::super::params::NetworkInspect<'a>>,
    ) -> Result<super::super::models::NetworkResource, Error> {
        let mut request_url = url::Url::parse(self.config.get_base_path())?;

        let mut request_path = request_url.path().to_owned();
        if request_path.ends_with('/') {
            request_path.pop();
        }
        request_path.push_str("/networks/{name}");
        request_path = request_path.replace("{name}", name);

        request_url.set_path(&request_path);

        let mut req_builder = self.config.req_builder("GET")?;

        if let Some(params) = params {
            let mut query_pairs = request_url.query_pairs_mut();
            if let Some(verbose) = params.verbose {
                query_pairs.append_pair("verbose", &verbose.to_string());
            }
            if let Some(scope) = params.scope {
                query_pairs.append_pair("scope", &scope);
            }
        }

        let hyper_uri: hyper::Uri = request_url.as_str().parse()?;
        req_builder = req_builder.uri(hyper_uri);
        let request = req_builder.body(String::new())?;
        request::execute_request_json(&*self.config, request).await
    }

    /// POST /networks/{name}/connect
    /// Connect container to network
    /// Connect a container to a network
    pub async fn network_connect(
        &self,
        name: &str,
        create: super::super::models::NetworkConnect,
    ) -> Result<(), Error> {
        let mut request_url = url::Url::parse(self.config.get_base_path())?;

        let mut request_path = request_url.path().to_owned();
        if request_path.ends_with('/') {
            request_path.pop();
        }
        request_path.push_str("/networks/{name}/connect");
        request_path = request_path.replace("{name}", name);

        request_url.set_path(&request_path);

        let mut req_builder = self.config.req_builder("POST")?;

        let hyper_uri: hyper::Uri = request_url.as_str().parse()?;
        req_builder = req_builder.uri(hyper_uri);
        let body = serde_json::to_string(&create)?;
        req_builder = req_builder.header(hyper::header::CONTENT_TYPE, "application/json");
        req_builder = req_builder.header(hyper::header::CONTENT_LENGTH, body.len());
        let request = req_builder.body(body)?;
        request::execute_request_unit(&*self.config, request).await
    }

    /// POST /networks/{name}/disconnect
    /// Disconnect container from network
    /// Disconnect a container from a network
    pub async fn network_disconnect(
        &self,
        name: &str,
        create: super::super::models::NetworkDisconnect,
    ) -> Result<(), Error> {
        let mut request_url = url::Url::parse(self.config.get_base_path())?;

        let mut request_path = request_url.path().to_owned();
        if request_path.ends_with('/') {
            request_path.pop();
        }
        request_path.push_str("/networks/{name}/disconnect");
        request_path = request_path.replace("{name}", name);

        request_url.set_path(&request_path);

        let mut req_builder = self.config.req_builder("POST")?;

        let hyper_uri: hyper::Uri = request_url.as_str().parse()?;
        req_builder = req_builder.uri(hyper_uri);
        let body = serde_json::to_string(&create)?;
        req_builder = req_builder.header(hyper::header::CONTENT_TYPE, "application/json");
        req_builder = req_builder.header(hyper::header::CONTENT_LENGTH, body.len());
        let request = req_builder.body(body)?;
        request::execute_request_unit(&*self.config, request).await
    }

    /// POST /networks/create
    /// Create network
    /// Create a network configuration
    pub async fn network_create(
        &self,
        create: super::super::models::NetworkCreateRequest,
    ) -> Result<super::super::models::NetworkCreate201, Error> {
        let mut request_url = url::Url::parse(self.config.get_base_path())?;

        let mut request_path = request_url.path().to_owned();
        if request_path.ends_with('/') {
            request_path.pop();
        }
        request_path.push_str("/networks/create");

        request_url.set_path(&request_path);

        let mut req_builder = self.config.req_builder("POST")?;

        let hyper_uri: hyper::Uri = request_url.as_str().parse()?;
        req_builder = req_builder.uri(hyper_uri);
        let body = serde_json::to_string(&create)?;
        req_builder = req_builder.header(hyper::header::CONTENT_TYPE, "application/json");
        req_builder = req_builder.header(hyper::header::CONTENT_LENGTH, body.len());
        let request = req_builder.body(body)?;
        request::execute_request_json(&*self.config, request).await
    }

    /// POST /networks/prune
    /// Delete unused networks
    /// Remove networks that do not have containers
    pub async fn network_prune<'a>(
        &self,
        params: Option<super::super::params::NetworkPrune<'a>>,
    ) -> Result<super::super::models::NetworkPrune200, Error> {
        let mut request_url = url::Url::parse(self.config.get_base_path())?;

        let mut request_path = request_url.path().to_owned();
        if request_path.ends_with('/') {
            request_path.pop();
        }
        request_path.push_str("/networks/prune");

        request_url.set_path(&request_path);

        let mut req_builder = self.config.req_builder("POST")?;

        if let Some(params) = params {
            let mut query_pairs = request_url.query_pairs_mut();
            if let Some(filters) = params.filters {
                query_pairs.append_pair("filters", &filters);
            }
        }

        let hyper_uri: hyper::Uri = request_url.as_str().parse()?;
        req_builder = req_builder.uri(hyper_uri);
        let request = req_builder.body(String::new())?;
        request::execute_request_json(&*self.config, request).await
    }
}

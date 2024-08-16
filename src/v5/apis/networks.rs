use crate::api_common::config::HasConfig;
use crate::api_common::request;
use crate::api_common::Error;
use std::future::Future;
use std::pin::Pin;
pub trait Networks: HasConfig + Send + Sync {
    /// DELETE /libpod/networks/{name}
    /// Remove a network
    /// Remove a configured network
    fn network_delete_libpod<'a>(
        &'a self,
        name: &'a str,
        params: Option<super::super::params::NetworkDeleteLibpod>,
    ) -> Pin<
        Box<
            dyn Future<Output = Result<Vec<super::super::models::NetworkRmReport>, Error>>
                + Send
                + 'a,
        >,
    > {
        Box::pin(async move {
            let mut request_url = url::Url::parse(self.get_config().get_base_path())?;
            let mut request_path = request_url.path().to_owned();
            if request_path.ends_with('/') {
                request_path.pop();
            }
            request_path.push_str("/libpod/networks/{name}");
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
            let request = req_builder.body(String::new())?;
            request::execute_request_json(self.get_config(), request).await
        })
    }
    /// POST /libpod/networks/{name}/connect
    /// Connect container to network
    /// Connect a container to a network.
    fn network_connect_libpod<'a>(
        &'a self,
        name: &'a str,
        create: super::super::models::NetworkConnectOptions,
    ) -> Pin<Box<dyn Future<Output = Result<(), Error>> + Send + 'a>> {
        Box::pin(async move {
            let mut request_url = url::Url::parse(self.get_config().get_base_path())?;
            let mut request_path = request_url.path().to_owned();
            if request_path.ends_with('/') {
                request_path.pop();
            }
            request_path.push_str("/libpod/networks/{name}/connect");
            request_path = request_path.replace("{name}", name);
            request_url.set_path(&request_path);
            let mut req_builder = self.get_config().req_builder("POST")?;
            let hyper_uri: hyper::Uri = request_url.as_str().parse()?;
            req_builder = req_builder.uri(hyper_uri);
            let body = serde_json::to_string(&create)?;
            req_builder = req_builder.header(hyper::header::CONTENT_TYPE, "application/json");
            req_builder = req_builder.header(hyper::header::CONTENT_LENGTH, body.len());
            let request = req_builder.body(body)?;
            request::execute_request_unit(self.get_config(), request).await
        })
    }
    /// POST /libpod/networks/{name}/disconnect
    /// Disconnect container from network
    /// Disconnect a container from a network.
    fn network_disconnect_libpod<'a>(
        &'a self,
        name: &'a str,
        create: super::super::models::NetworkDisconnect,
    ) -> Pin<Box<dyn Future<Output = Result<(), Error>> + Send + 'a>> {
        Box::pin(async move {
            let mut request_url = url::Url::parse(self.get_config().get_base_path())?;
            let mut request_path = request_url.path().to_owned();
            if request_path.ends_with('/') {
                request_path.pop();
            }
            request_path.push_str("/libpod/networks/{name}/disconnect");
            request_path = request_path.replace("{name}", name);
            request_url.set_path(&request_path);
            let mut req_builder = self.get_config().req_builder("POST")?;
            let hyper_uri: hyper::Uri = request_url.as_str().parse()?;
            req_builder = req_builder.uri(hyper_uri);
            let body = serde_json::to_string(&create)?;
            req_builder = req_builder.header(hyper::header::CONTENT_TYPE, "application/json");
            req_builder = req_builder.header(hyper::header::CONTENT_LENGTH, body.len());
            let request = req_builder.body(body)?;
            request::execute_request_unit(self.get_config(), request).await
        })
    }
    /// GET /libpod/networks/{name}/exists
    /// Network exists
    /// Check if network exists
    fn network_exists_libpod<'a>(
        &'a self,
        name: &'a str,
    ) -> Pin<Box<dyn Future<Output = Result<(), Error>> + Send + 'a>> {
        Box::pin(async move {
            let mut request_url = url::Url::parse(self.get_config().get_base_path())?;
            let mut request_path = request_url.path().to_owned();
            if request_path.ends_with('/') {
                request_path.pop();
            }
            request_path.push_str("/libpod/networks/{name}/exists");
            request_path = request_path.replace("{name}", name);
            request_url.set_path(&request_path);
            let mut req_builder = self.get_config().req_builder("GET")?;
            let hyper_uri: hyper::Uri = request_url.as_str().parse()?;
            req_builder = req_builder.uri(hyper_uri);
            let request = req_builder.body(String::new())?;
            request::execute_request_unit(self.get_config(), request).await
        })
    }
    /// GET /libpod/networks/{name}/json
    /// Inspect a network
    /// Display configuration for a network.
    fn network_inspect_libpod<'a>(
        &'a self,
        name: &'a str,
    ) -> Pin<
        Box<
            dyn Future<Output = Result<super::super::models::NetworkInspectReport, Error>>
                + Send
                + 'a,
        >,
    > {
        Box::pin(async move {
            let mut request_url = url::Url::parse(self.get_config().get_base_path())?;
            let mut request_path = request_url.path().to_owned();
            if request_path.ends_with('/') {
                request_path.pop();
            }
            request_path.push_str("/libpod/networks/{name}/json");
            request_path = request_path.replace("{name}", name);
            request_url.set_path(&request_path);
            let mut req_builder = self.get_config().req_builder("GET")?;
            let hyper_uri: hyper::Uri = request_url.as_str().parse()?;
            req_builder = req_builder.uri(hyper_uri);
            let request = req_builder.body(String::new())?;
            request::execute_request_json(self.get_config(), request).await
        })
    }
    /// POST /libpod/networks/{name}/update
    /// Update existing podman network
    /// Update existing podman network
    fn network_update_libpod<'a>(
        &'a self,
        name: &'a str,
        update: super::super::models::NetworkUpdateOptions,
    ) -> Pin<Box<dyn Future<Output = Result<(), Error>> + Send + 'a>> {
        Box::pin(async move {
            let mut request_url = url::Url::parse(self.get_config().get_base_path())?;
            let mut request_path = request_url.path().to_owned();
            if request_path.ends_with('/') {
                request_path.pop();
            }
            request_path.push_str("/libpod/networks/{name}/update");
            request_path = request_path.replace("{name}", name);
            request_url.set_path(&request_path);
            let mut req_builder = self.get_config().req_builder("POST")?;
            let hyper_uri: hyper::Uri = request_url.as_str().parse()?;
            req_builder = req_builder.uri(hyper_uri);
            let body = serde_json::to_string(&update)?;
            req_builder = req_builder.header(hyper::header::CONTENT_TYPE, "application/json");
            req_builder = req_builder.header(hyper::header::CONTENT_LENGTH, body.len());
            let request = req_builder.body(body)?;
            request::execute_request_unit(self.get_config(), request).await
        })
    }
    /// POST /libpod/networks/create
    /// Create network
    /// Create a new network configuration
    fn network_create_libpod<'a>(
        &'a self,
        create: super::super::models::NetworkCreateLibpod,
    ) -> Pin<Box<dyn Future<Output = Result<super::super::models::Network, Error>> + Send + 'a>>
    {
        Box::pin(async move {
            let mut request_url = url::Url::parse(self.get_config().get_base_path())?;
            let mut request_path = request_url.path().to_owned();
            if request_path.ends_with('/') {
                request_path.pop();
            }
            request_path.push_str("/libpod/networks/create");
            request_url.set_path(&request_path);
            let mut req_builder = self.get_config().req_builder("POST")?;
            let hyper_uri: hyper::Uri = request_url.as_str().parse()?;
            req_builder = req_builder.uri(hyper_uri);
            let body = serde_json::to_string(&create)?;
            req_builder = req_builder.header(hyper::header::CONTENT_TYPE, "application/json");
            req_builder = req_builder.header(hyper::header::CONTENT_LENGTH, body.len());
            let request = req_builder.body(body)?;
            request::execute_request_json(self.get_config(), request).await
        })
    }
    /// GET /libpod/networks/json
    /// List networks
    /// Display summary of network configurations.
    ///   - In a 200 response, all of the fields named Bytes are returned as a Base64 encoded string.
    fn network_list_libpod<'a>(
        &'a self,
        params: Option<super::super::params::NetworkListLibpod<'a>>,
    ) -> Pin<Box<dyn Future<Output = Result<Vec<super::super::models::Network>, Error>> + Send + 'a>>
    {
        Box::pin(async move {
            let mut request_url = url::Url::parse(self.get_config().get_base_path())?;
            let mut request_path = request_url.path().to_owned();
            if request_path.ends_with('/') {
                request_path.pop();
            }
            request_path.push_str("/libpod/networks/json");
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
            let request = req_builder.body(String::new())?;
            request::execute_request_json(self.get_config(), request).await
        })
    }
    /// POST /libpod/networks/prune
    /// Delete unused networks
    /// Remove networks that do not have containers
    fn network_prune_libpod<'a>(
        &'a self,
        params: Option<super::super::params::NetworkPruneLibpod<'a>>,
    ) -> Pin<
        Box<
            dyn Future<Output = Result<Vec<super::super::models::NetworkPruneReport>, Error>>
                + Send
                + 'a,
        >,
    > {
        Box::pin(async move {
            let mut request_url = url::Url::parse(self.get_config().get_base_path())?;
            let mut request_path = request_url.path().to_owned();
            if request_path.ends_with('/') {
                request_path.pop();
            }
            request_path.push_str("/libpod/networks/prune");
            request_url.set_path(&request_path);
            let mut req_builder = self.get_config().req_builder("POST")?;
            if let Some(params) = params {
                let mut query_pairs = request_url.query_pairs_mut();
                if let Some(filters) = params.filters {
                    query_pairs.append_pair("filters", filters);
                }
            }
            let hyper_uri: hyper::Uri = request_url.as_str().parse()?;
            req_builder = req_builder.uri(hyper_uri);
            let request = req_builder.body(String::new())?;
            request::execute_request_json(self.get_config(), request).await
        })
    }
}

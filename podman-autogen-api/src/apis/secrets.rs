use std::sync::Arc;

use super::super::config::ClientConfig;
use super::super::request;
use super::super::Error;

/// Actions related to secrets
pub struct Secrets {
    config: Arc<dyn ClientConfig>,
}

impl Secrets {
    pub fn new(config: Arc<dyn ClientConfig>) -> Secrets {
        Secrets { config }
    }

    /// DELETE /libpod/secrets/{name}
    /// Remove secret
    pub async fn secret_delete_libpod(
        &self,
        name: &str,
        params: Option<super::super::params::SecretDeleteLibpod>,
    ) -> Result<(), Error> {
        let mut request_url = url::Url::parse(self.config.get_base_path())?;

        let mut request_path = request_url.path().to_owned();
        if request_path.ends_with('/') {
            request_path.pop();
        }
        request_path.push_str("/libpod/secrets/{name}");
        request_path = request_path.replace("{name}", name);

        request_url.set_path(&request_path);

        let mut req_builder = self.config.req_builder("DELETE")?;

        if let Some(params) = params {
            let mut query_pairs = request_url.query_pairs_mut();
            if let Some(all) = params.all {
                query_pairs.append_pair("all", &all.to_string());
            }
        }

        let hyper_uri: hyper::Uri = request_url.as_str().parse()?;
        req_builder = req_builder.uri(hyper_uri);
        let request = req_builder.body(String::new())?;
        request::execute_request_unit(&*self.config, request).await
    }

    /// GET /libpod/secrets/{name}/exists
    /// Secret exists
    pub async fn secret_exists_libpod(&self, name: &str) -> Result<(), Error> {
        let mut request_url = url::Url::parse(self.config.get_base_path())?;

        let mut request_path = request_url.path().to_owned();
        if request_path.ends_with('/') {
            request_path.pop();
        }
        request_path.push_str("/libpod/secrets/{name}/exists");
        request_path = request_path.replace("{name}", name);

        request_url.set_path(&request_path);

        let mut req_builder = self.config.req_builder("GET")?;

        let hyper_uri: hyper::Uri = request_url.as_str().parse()?;
        req_builder = req_builder.uri(hyper_uri);
        let request = req_builder.body(String::new())?;
        request::execute_request_unit(&*self.config, request).await
    }

    /// GET /libpod/secrets/{name}/json
    /// Inspect secret
    pub async fn secret_inspect_libpod(
        &self,
        name: &str,
        params: Option<super::super::params::SecretInspectLibpod>,
    ) -> Result<super::super::models::SecretInfoReport, Error> {
        let mut request_url = url::Url::parse(self.config.get_base_path())?;

        let mut request_path = request_url.path().to_owned();
        if request_path.ends_with('/') {
            request_path.pop();
        }
        request_path.push_str("/libpod/secrets/{name}/json");
        request_path = request_path.replace("{name}", name);

        request_url.set_path(&request_path);

        let mut req_builder = self.config.req_builder("GET")?;

        if let Some(params) = params {
            let mut query_pairs = request_url.query_pairs_mut();
            if let Some(showsecret) = params.showsecret {
                query_pairs.append_pair("showsecret", &showsecret.to_string());
            }
        }

        let hyper_uri: hyper::Uri = request_url.as_str().parse()?;
        req_builder = req_builder.uri(hyper_uri);
        let request = req_builder.body(String::new())?;
        request::execute_request_json(&*self.config, request).await
    }

    /// POST /libpod/secrets/create
    /// Create a secret
    pub async fn secret_create_libpod<'a>(
        &self,
        params: Option<super::super::params::SecretCreateLibpod<'a>>,
        request: String,
    ) -> Result<super::super::models::SecretCreateResponse, Error> {
        let mut request_url = url::Url::parse(self.config.get_base_path())?;

        let mut request_path = request_url.path().to_owned();
        if request_path.ends_with('/') {
            request_path.pop();
        }
        request_path.push_str("/libpod/secrets/create");

        request_url.set_path(&request_path);

        let mut req_builder = self.config.req_builder("POST")?;

        if let Some(params) = params {
            let mut query_pairs = request_url.query_pairs_mut();
            query_pairs.append_pair("name", &params.name);
            if let Some(driver) = params.driver {
                query_pairs.append_pair("driver", &driver);
            }
            if let Some(driveropts) = params.driveropts {
                query_pairs.append_pair("driveropts", &driveropts);
            }
            if let Some(labels) = params.labels {
                query_pairs.append_pair("labels", &labels);
            }
        }

        let hyper_uri: hyper::Uri = request_url.as_str().parse()?;
        req_builder = req_builder.uri(hyper_uri);
        let body = serde_json::to_string(&request)?;
        req_builder = req_builder.header(hyper::header::CONTENT_TYPE, "application/json");
        req_builder = req_builder.header(hyper::header::CONTENT_LENGTH, body.len());
        let request = req_builder.body(body)?;
        request::execute_request_json(&*self.config, request).await
    }

    /// GET /libpod/secrets/json
    /// List secrets
    /// Returns a list of secrets
    pub async fn secret_list_libpod<'a>(
        &self,
        params: Option<super::super::params::SecretListLibpod<'a>>,
    ) -> Result<Vec<super::super::models::SecretInfoReport>, Error> {
        let mut request_url = url::Url::parse(self.config.get_base_path())?;

        let mut request_path = request_url.path().to_owned();
        if request_path.ends_with('/') {
            request_path.pop();
        }
        request_path.push_str("/libpod/secrets/json");

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
}

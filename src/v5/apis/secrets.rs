use crate::api_common::config::HasConfig;
use crate::api_common::request;
use crate::api_common::Error;
use std::future::Future;
use std::pin::Pin;
pub trait Secrets: HasConfig + Send + Sync {
    /// DELETE /libpod/secrets/{name}
    ///
    /// Remove secret
    fn secret_delete_libpod<'a>(
        &'a self,
        name: &'a str,
        params: Option<crate::v5::params::SecretDeleteLibpod>,
    ) -> Pin<Box<dyn Future<Output = Result<(), Error>> + Send + 'a>> {
        Box::pin(request::execute_request_unit(
            self.get_config(),
            (|| {
                let mut request_url = url::Url::parse(self.get_config().get_base_path())?;
                let mut request_path = request_url.path().to_owned();
                if request_path.ends_with('/') {
                    request_path.pop();
                }
                request_path.push_str("/libpod/secrets/{name}");
                request_path = request_path.replace("{name}", name);
                request_url.set_path(&request_path);
                let mut req_builder = self.get_config().req_builder("DELETE")?;
                if let Some(params) = params {
                    let mut query_pairs = request_url.query_pairs_mut();
                    if let Some(all) = params.all {
                        query_pairs.append_pair("all", &all.to_string());
                    }
                }
                let hyper_uri: hyper::Uri = request_url.as_str().parse()?;
                req_builder = req_builder.uri(hyper_uri);
                Ok(req_builder.body(String::new())?)
            })(),
        ))
    }
    /// GET /libpod/secrets/{name}/exists
    ///
    /// Secret exists
    fn secret_exists_libpod<'a>(
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
                request_path.push_str("/libpod/secrets/{name}/exists");
                request_path = request_path.replace("{name}", name);
                request_url.set_path(&request_path);
                let mut req_builder = self.get_config().req_builder("GET")?;
                let hyper_uri: hyper::Uri = request_url.as_str().parse()?;
                req_builder = req_builder.uri(hyper_uri);
                Ok(req_builder.body(String::new())?)
            })(),
        ))
    }
    /// GET /libpod/secrets/{name}/json
    ///
    /// Inspect secret
    fn secret_inspect_libpod<'a>(
        &'a self,
        name: &'a str,
        params: Option<crate::v5::params::SecretInspectLibpod>,
    ) -> Pin<Box<dyn Future<Output = Result<crate::v5::models::SecretInfoReport, Error>> + Send + 'a>>
    {
        Box::pin(request::execute_request_json(
            self.get_config(),
            (|| {
                let mut request_url = url::Url::parse(self.get_config().get_base_path())?;
                let mut request_path = request_url.path().to_owned();
                if request_path.ends_with('/') {
                    request_path.pop();
                }
                request_path.push_str("/libpod/secrets/{name}/json");
                request_path = request_path.replace("{name}", name);
                request_url.set_path(&request_path);
                let mut req_builder = self.get_config().req_builder("GET")?;
                if let Some(params) = params {
                    let mut query_pairs = request_url.query_pairs_mut();
                    if let Some(showsecret) = params.showsecret {
                        query_pairs.append_pair("showsecret", &showsecret.to_string());
                    }
                }
                let hyper_uri: hyper::Uri = request_url.as_str().parse()?;
                req_builder = req_builder.uri(hyper_uri);
                Ok(req_builder.body(String::new())?)
            })(),
        ))
    }
    /// POST /libpod/secrets/create
    ///
    /// Create a secret
    fn secret_create_libpod<'a>(
        &'a self,
        params: Option<crate::v5::params::SecretCreateLibpod<'a>>,
        request: String,
    ) -> Pin<
        Box<
            dyn Future<Output = Result<crate::v5::models::SecretCreateResponse, Error>> + Send + 'a,
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
                request_path.push_str("/libpod/secrets/create");
                request_url.set_path(&request_path);
                let mut req_builder = self.get_config().req_builder("POST")?;
                if let Some(params) = params {
                    let mut query_pairs = request_url.query_pairs_mut();
                    query_pairs.append_pair("name", params.name);
                    if let Some(driver) = params.driver {
                        query_pairs.append_pair("driver", driver);
                    }
                    if let Some(driveropts) = params.driveropts {
                        query_pairs.append_pair("driveropts", driveropts);
                    }
                    if let Some(labels) = params.labels {
                        query_pairs.append_pair("labels", labels);
                    }
                }
                let hyper_uri: hyper::Uri = request_url.as_str().parse()?;
                req_builder = req_builder.uri(hyper_uri);
                let body = serde_json::to_string(&request)?;
                req_builder = req_builder.header(hyper::header::CONTENT_TYPE, "application/json");
                req_builder = req_builder.header(hyper::header::CONTENT_LENGTH, body.len());
                Ok(req_builder.body(body)?)
            })(),
        ))
    }
    /// GET /libpod/secrets/json
    ///
    /// List secrets
    ///
    /// Returns a list of secrets
    fn secret_list_libpod<'a>(
        &'a self,
        params: Option<crate::v5::params::SecretListLibpod<'a>>,
    ) -> Pin<
        Box<
            dyn Future<Output = Result<Vec<crate::v5::models::SecretInfoReport>, Error>>
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
                request_path.push_str("/libpod/secrets/json");
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
}

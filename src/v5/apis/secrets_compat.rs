use crate::api_common::config::HasConfig;
use crate::api_common::request;
use crate::api_common::Error;
use http::request::Builder;
use std::future::Future;
use std::pin::Pin;
pub trait SecretsCompat: HasConfig + Send + Sync {
    /// GET /secrets
    ///
    /// List secrets
    ///
    /// Returns a list of secrets
    fn secret_list<'a>(
        &'a self,
        params: Option<crate::v5::params::SecretList<'a>>,
    ) -> Pin<
        Box<
            dyn Future<Output = Result<Vec<crate::v5::models::SecretInfoReportCompat>, Error>>
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
                request_path.push_str("/secrets");
                request_url.set_path(&request_path);
                if let Some(params) = &params {
                    let mut query_pairs = request_url.query_pairs_mut();
                    if let Some(filters) = params.filters {
                        query_pairs.append_pair("filters", filters);
                    }
                }
                let hyper_uri: hyper::Uri = request_url.as_str().parse()?;
                req_builder = req_builder.uri(hyper_uri);
                Ok(req_builder.body(String::new())?)
            },
        ))
    }
    /// DELETE /secrets/{name}
    ///
    /// Remove secret
    fn secret_delete<'a>(
        &'a self,
        name: &'a str,
    ) -> Pin<Box<dyn Future<Output = Result<(), Error>> + Send + 'a>> {
        Box::pin(request::execute_request_unit(
            self.get_config(),
            move |mut req_builder: Builder| {
                req_builder = req_builder.method("DELETE");
                let mut request_url = url::Url::parse(self.get_config().get_base_path())?;
                let mut request_path = request_url.path().to_owned();
                if request_path.ends_with('/') {
                    request_path.pop();
                }
                request_path.push_str("/secrets/{name}");
                request_path = request_path.replace("{name}", name);
                request_url.set_path(&request_path);
                let hyper_uri: hyper::Uri = request_url.as_str().parse()?;
                req_builder = req_builder.uri(hyper_uri);
                Ok(req_builder.body(String::new())?)
            },
        ))
    }
    /// GET /secrets/{name}
    ///
    /// Inspect secret
    fn secret_inspect<'a>(
        &'a self,
        name: &'a str,
    ) -> Pin<
        Box<
            dyn Future<Output = Result<crate::v5::models::SecretInfoReportCompat, Error>>
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
                request_path.push_str("/secrets/{name}");
                request_path = request_path.replace("{name}", name);
                request_url.set_path(&request_path);
                let hyper_uri: hyper::Uri = request_url.as_str().parse()?;
                req_builder = req_builder.uri(hyper_uri);
                Ok(req_builder.body(String::new())?)
            },
        ))
    }
    /// POST /secrets/create
    ///
    /// Create a secret
    fn secret_create<'a>(
        &'a self,
        create: crate::v5::models::SecretCreate,
    ) -> Pin<
        Box<
            dyn Future<Output = Result<crate::v5::models::SecretCreateResponse, Error>> + Send + 'a,
        >,
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
                request_path.push_str("/secrets/create");
                request_url.set_path(&request_path);
                let hyper_uri: hyper::Uri = request_url.as_str().parse()?;
                req_builder = req_builder.uri(hyper_uri);
                let body = serde_json::to_string(&create)?;
                req_builder = req_builder.header(hyper::header::CONTENT_TYPE, "application/json");
                req_builder = req_builder.header(hyper::header::CONTENT_LENGTH, body.len());
                Ok(req_builder.body(body)?)
            },
        ))
    }
}

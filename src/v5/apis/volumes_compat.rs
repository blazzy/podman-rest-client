use crate::api_common::config::HasConfig;
use crate::api_common::request;
use crate::api_common::Error;
use std::future::Future;
use std::pin::Pin;
pub trait VolumesCompat: HasConfig + Send + Sync {
    /// GET /volumes
    /// List volumes
    /// Returns a list of volume
    fn volume_list<'a>(
        &'a self,
        params: Option<super::super::params::VolumeList<'a>>,
    ) -> Pin<Box<dyn Future<Output = Result<super::super::models::ListResponse, Error>> + Send + 'a>>
    {
        Box::pin(request::execute_request_json(
            self.get_config(),
            (|| {
                let mut request_url = url::Url::parse(self.get_config().get_base_path())?;
                let mut request_path = request_url.path().to_owned();
                if request_path.ends_with('/') {
                    request_path.pop();
                }
                request_path.push_str("/volumes");
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
    /// DELETE /volumes/{name}
    /// Remove volume
    fn volume_delete<'a>(
        &'a self,
        name: &'a str,
        params: Option<super::super::params::VolumeDelete>,
    ) -> Pin<Box<dyn Future<Output = Result<(), Error>> + Send + 'a>> {
        Box::pin(request::execute_request_unit(
            self.get_config(),
            (|| {
                let mut request_url = url::Url::parse(self.get_config().get_base_path())?;
                let mut request_path = request_url.path().to_owned();
                if request_path.ends_with('/') {
                    request_path.pop();
                }
                request_path.push_str("/volumes/{name}");
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
    /// GET /volumes/{name}
    /// Inspect volume
    fn volume_inspect<'a>(
        &'a self,
        name: &'a str,
    ) -> Pin<Box<dyn Future<Output = Result<super::super::models::Volume, Error>> + Send + 'a>>
    {
        Box::pin(request::execute_request_json(
            self.get_config(),
            (|| {
                let mut request_url = url::Url::parse(self.get_config().get_base_path())?;
                let mut request_path = request_url.path().to_owned();
                if request_path.ends_with('/') {
                    request_path.pop();
                }
                request_path.push_str("/volumes/{name}");
                request_path = request_path.replace("{name}", name);
                request_url.set_path(&request_path);
                let mut req_builder = self.get_config().req_builder("GET")?;
                let hyper_uri: hyper::Uri = request_url.as_str().parse()?;
                req_builder = req_builder.uri(hyper_uri);
                Ok(req_builder.body(String::new())?)
            })(),
        ))
    }
    /// POST /volumes/create
    /// Create a volume
    fn volume_create<'a>(
        &'a self,
        create: super::super::models::VolumeCreate,
    ) -> Pin<Box<dyn Future<Output = Result<super::super::models::Volume, Error>> + Send + 'a>>
    {
        Box::pin(request::execute_request_json(
            self.get_config(),
            (|| {
                let mut request_url = url::Url::parse(self.get_config().get_base_path())?;
                let mut request_path = request_url.path().to_owned();
                if request_path.ends_with('/') {
                    request_path.pop();
                }
                request_path.push_str("/volumes/create");
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
    /// POST /volumes/prune
    /// Prune volumes
    fn volume_prune<'a>(
        &'a self,
        params: Option<super::super::params::VolumePrune<'a>>,
    ) -> Pin<
        Box<
            dyn Future<Output = Result<super::super::models::VolumesPruneReport, Error>>
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
                request_path.push_str("/volumes/prune");
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
                Ok(req_builder.body(String::new())?)
            })(),
        ))
    }
}

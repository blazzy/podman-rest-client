use crate::api_common::config::HasConfig;
use crate::api_common::request;
use crate::api_common::Error;
use std::future::Future;
use std::pin::Pin;
pub trait Volumes: HasConfig + Send + Sync {
    /// DELETE /libpod/volumes/{name}
    /// Remove volume
    fn volume_delete_libpod<'a>(
        &'a self,
        name: &'a str,
        params: Option<super::super::params::VolumeDeleteLibpod>,
    ) -> Pin<Box<dyn Future<Output = Result<(), Error>> + Send + 'a>> {
        Box::pin(request::execute_request_unit(
            self.get_config(),
            (|| {
                let mut request_url = url::Url::parse(self.get_config().get_base_path())?;
                let mut request_path = request_url.path().to_owned();
                if request_path.ends_with('/') {
                    request_path.pop();
                }
                request_path.push_str("/libpod/volumes/{name}");
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
    /// GET /libpod/volumes/{name}/exists
    /// Volume exists
    /// Check if a volume exists
    fn volume_exists_libpod<'a>(
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
                request_path.push_str("/libpod/volumes/{name}/exists");
                request_path = request_path.replace("{name}", name);
                request_url.set_path(&request_path);
                let mut req_builder = self.get_config().req_builder("GET")?;
                let hyper_uri: hyper::Uri = request_url.as_str().parse()?;
                req_builder = req_builder.uri(hyper_uri);
                Ok(req_builder.body(String::new())?)
            })(),
        ))
    }
    /// GET /libpod/volumes/{name}/json
    /// Inspect volume
    fn volume_inspect_libpod<'a>(
        &'a self,
        name: &'a str,
    ) -> Pin<
        Box<
            dyn Future<Output = Result<super::super::models::VolumeConfigResponse, Error>>
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
                request_path.push_str("/libpod/volumes/{name}/json");
                request_path = request_path.replace("{name}", name);
                request_url.set_path(&request_path);
                let mut req_builder = self.get_config().req_builder("GET")?;
                let hyper_uri: hyper::Uri = request_url.as_str().parse()?;
                req_builder = req_builder.uri(hyper_uri);
                Ok(req_builder.body(String::new())?)
            })(),
        ))
    }
    /// POST /libpod/volumes/create
    /// Create a volume
    fn volume_create_libpod<'a>(
        &'a self,
        create: super::super::models::VolumeCreateOptions,
    ) -> Pin<
        Box<
            dyn Future<Output = Result<super::super::models::VolumeConfigResponse, Error>>
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
                request_path.push_str("/libpod/volumes/create");
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
    /// GET /libpod/volumes/json
    /// List volumes
    /// Returns a list of volumes
    fn volume_list_libpod<'a>(
        &'a self,
        params: Option<super::super::params::VolumeListLibpod<'a>>,
    ) -> Pin<
        Box<
            dyn Future<Output = Result<Vec<super::super::models::VolumeConfigResponse>, Error>>
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
                request_path.push_str("/libpod/volumes/json");
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
    /// POST /libpod/volumes/prune
    /// Prune volumes
    fn volume_prune_libpod<'a>(
        &'a self,
        params: Option<super::super::params::VolumePruneLibpod<'a>>,
    ) -> Pin<
        Box<dyn Future<Output = Result<Vec<super::super::models::PruneReport>, Error>> + Send + 'a>,
    > {
        Box::pin(request::execute_request_json(
            self.get_config(),
            (|| {
                let mut request_url = url::Url::parse(self.get_config().get_base_path())?;
                let mut request_path = request_url.path().to_owned();
                if request_path.ends_with('/') {
                    request_path.pop();
                }
                request_path.push_str("/libpod/volumes/prune");
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

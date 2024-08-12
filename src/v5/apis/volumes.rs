use crate::v5::config::HasConfig;
use crate::v5::request;
use crate::v5::Error;
#[async_trait::async_trait]
pub trait Volumes: HasConfig + Send + Sync {
    /// DELETE /libpod/volumes/{name}
    /// Remove volume
    async fn volume_delete_libpod(
        &self,
        name: &str,
        params: Option<super::super::params::VolumeDeleteLibpod>,
    ) -> Result<(), Error> {
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
        let request = req_builder.body(String::new())?;
        request::execute_request_unit(self.get_config(), request).await
    }
    /// GET /libpod/volumes/{name}/exists
    /// Volume exists
    /// Check if a volume exists
    async fn volume_exists_libpod(&self, name: &str) -> Result<(), Error> {
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
        let request = req_builder.body(String::new())?;
        request::execute_request_unit(self.get_config(), request).await
    }
    /// GET /libpod/volumes/{name}/json
    /// Inspect volume
    async fn volume_inspect_libpod(
        &self,
        name: &str,
    ) -> Result<super::super::models::VolumeConfigResponse, Error> {
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
        let request = req_builder.body(String::new())?;
        request::execute_request_json(self.get_config(), request).await
    }
    /// POST /libpod/volumes/create
    /// Create a volume
    async fn volume_create_libpod(
        &self,
        create: super::super::models::VolumeCreateOptions,
    ) -> Result<super::super::models::VolumeConfigResponse, Error> {
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
        let request = req_builder.body(body)?;
        request::execute_request_json(self.get_config(), request).await
    }
    /// GET /libpod/volumes/json
    /// List volumes
    /// Returns a list of volumes
    async fn volume_list_libpod<'a>(
        &self,
        params: Option<super::super::params::VolumeListLibpod<'a>>,
    ) -> Result<Vec<super::super::models::VolumeConfigResponse>, Error> {
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
        let request = req_builder.body(String::new())?;
        request::execute_request_json(self.get_config(), request).await
    }
    /// POST /libpod/volumes/prune
    /// Prune volumes
    async fn volume_prune_libpod<'a>(
        &self,
        params: Option<super::super::params::VolumePruneLibpod<'a>>,
    ) -> Result<Vec<super::super::models::PruneReport>, Error> {
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
        let request = req_builder.body(String::new())?;
        request::execute_request_json(self.get_config(), request).await
    }
}

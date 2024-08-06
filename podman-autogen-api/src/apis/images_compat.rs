use std::sync::Arc;

use super::super::config::ClientConfig;
use super::super::request;
use super::super::Error;

/// Actions related to images for the compatibility endpoints
pub struct ImagesCompat {
    config: Arc<dyn ClientConfig>,
}

impl ImagesCompat {
    pub fn new(config: Arc<dyn ClientConfig>) -> ImagesCompat {
        ImagesCompat { config }
    }

    /// POST /build
    /// Create image
    /// Build an image from the given Dockerfile(s)
    pub async fn image_build<'a>(
        &self,
        params: Option<super::super::params::ImageBuild<'a>>,
        input_stream: String,
    ) -> Result<super::super::models::ImageBuild200, Error> {
        let mut request_url = url::Url::parse(self.config.get_base_path())?;

        let mut request_path = request_url.path().to_owned();
        if request_path.ends_with('/') {
            request_path.pop();
        }
        request_path.push_str("/build");

        request_url.set_path(&request_path);

        let mut req_builder = self.config.req_builder("POST")?;

        if let Some(params) = params {
            let mut query_pairs = request_url.query_pairs_mut();
            if let Some(dockerfile) = params.dockerfile {
                query_pairs.append_pair("dockerfile", &dockerfile);
            }
            if let Some(t) = params.t {
                query_pairs.append_pair("t", &t);
            }
            if let Some(extrahosts) = params.extrahosts {
                query_pairs.append_pair("extrahosts", &extrahosts);
            }
            if let Some(remote) = params.remote {
                query_pairs.append_pair("remote", &remote);
            }
            if let Some(q) = params.q {
                query_pairs.append_pair("q", &q.to_string());
            }
            if let Some(nocache) = params.nocache {
                query_pairs.append_pair("nocache", &nocache.to_string());
            }
            if let Some(cachefrom) = params.cachefrom {
                query_pairs.append_pair("cachefrom", &cachefrom);
            }
            if let Some(pull) = params.pull {
                query_pairs.append_pair("pull", &pull.to_string());
            }
            if let Some(rm) = params.rm {
                query_pairs.append_pair("rm", &rm.to_string());
            }
            if let Some(forcerm) = params.forcerm {
                query_pairs.append_pair("forcerm", &forcerm.to_string());
            }
            if let Some(memory) = params.memory {
                query_pairs.append_pair("memory", &memory.to_string());
            }
            if let Some(memswap) = params.memswap {
                query_pairs.append_pair("memswap", &memswap.to_string());
            }
            if let Some(cpushares) = params.cpushares {
                query_pairs.append_pair("cpushares", &cpushares.to_string());
            }
            if let Some(cpusetcpus) = params.cpusetcpus {
                query_pairs.append_pair("cpusetcpus", &cpusetcpus);
            }
            if let Some(cpuperiod) = params.cpuperiod {
                query_pairs.append_pair("cpuperiod", &cpuperiod.to_string());
            }
            if let Some(cpuquota) = params.cpuquota {
                query_pairs.append_pair("cpuquota", &cpuquota.to_string());
            }
            if let Some(buildargs) = params.buildargs {
                query_pairs.append_pair("buildargs", &buildargs);
            }
            if let Some(shmsize) = params.shmsize {
                query_pairs.append_pair("shmsize", &shmsize.to_string());
            }
            if let Some(squash) = params.squash {
                query_pairs.append_pair("squash", &squash.to_string());
            }
            if let Some(labels) = params.labels {
                query_pairs.append_pair("labels", &labels);
            }
            if let Some(networkmode) = params.networkmode {
                query_pairs.append_pair("networkmode", &networkmode);
            }
            if let Some(platform) = params.platform {
                query_pairs.append_pair("platform", &platform);
            }
            if let Some(target) = params.target {
                query_pairs.append_pair("target", &target);
            }
            if let Some(outputs) = params.outputs {
                query_pairs.append_pair("outputs", &outputs);
            }

            if let Some(content_type) = params.content_type {
                req_builder = req_builder.header("Content-Type", content_type);
            }
            if let Some(x_registry_config) = params.x_registry_config {
                req_builder = req_builder.header("X-Registry-Config", x_registry_config);
            }
        }

        let hyper_uri: hyper::Uri = request_url.as_str().parse()?;
        req_builder = req_builder.uri(hyper_uri);
        let body = serde_json::to_string(&input_stream)?;
        req_builder = req_builder.header(hyper::header::CONTENT_TYPE, "application/json");
        req_builder = req_builder.header(hyper::header::CONTENT_LENGTH, body.len());
        let request = req_builder.body(body)?;
        request::execute_request_json(&*self.config, request).await
    }

    /// DELETE /images/{name}
    /// Remove Image
    /// Delete an image from local storage
    pub async fn image_delete(
        &self,
        name: &str,
        params: Option<super::super::params::ImageDelete>,
    ) -> Result<Vec<super::super::models::ImageDeleteResponseItems>, Error> {
        let mut request_url = url::Url::parse(self.config.get_base_path())?;

        let mut request_path = request_url.path().to_owned();
        if request_path.ends_with('/') {
            request_path.pop();
        }
        request_path.push_str("/images/{name}");
        request_path = request_path.replace("{name}", name);

        request_url.set_path(&request_path);

        let mut req_builder = self.config.req_builder("DELETE")?;

        if let Some(params) = params {
            let mut query_pairs = request_url.query_pairs_mut();
            if let Some(force) = params.force {
                query_pairs.append_pair("force", &force.to_string());
            }
            if let Some(noprune) = params.noprune {
                query_pairs.append_pair("noprune", &noprune.to_string());
            }
        }

        let hyper_uri: hyper::Uri = request_url.as_str().parse()?;
        req_builder = req_builder.uri(hyper_uri);
        let request = req_builder.body(String::new())?;
        request::execute_request_json(&*self.config, request).await
    }

    /// GET /images/{name}/get
    /// Export an image
    /// Export an image in tarball format
    pub async fn image_get(&self, name: &str) -> Result<String, Error> {
        let mut request_url = url::Url::parse(self.config.get_base_path())?;

        let mut request_path = request_url.path().to_owned();
        if request_path.ends_with('/') {
            request_path.pop();
        }
        request_path.push_str("/images/{name}/get");
        request_path = request_path.replace("{name}", name);

        request_url.set_path(&request_path);

        let mut req_builder = self.config.req_builder("GET")?;

        let hyper_uri: hyper::Uri = request_url.as_str().parse()?;
        req_builder = req_builder.uri(hyper_uri);
        let request = req_builder.body(String::new())?;
        request::execute_request_json(&*self.config, request).await
    }

    /// GET /images/{name}/history
    /// History of an image
    /// Return parent layers of an image.
    pub async fn image_history(
        &self,
        name: &str,
    ) -> Result<super::super::models::HistoryResponse, Error> {
        let mut request_url = url::Url::parse(self.config.get_base_path())?;

        let mut request_path = request_url.path().to_owned();
        if request_path.ends_with('/') {
            request_path.pop();
        }
        request_path.push_str("/images/{name}/history");
        request_path = request_path.replace("{name}", name);

        request_url.set_path(&request_path);

        let mut req_builder = self.config.req_builder("GET")?;

        let hyper_uri: hyper::Uri = request_url.as_str().parse()?;
        req_builder = req_builder.uri(hyper_uri);
        let request = req_builder.body(String::new())?;
        request::execute_request_json(&*self.config, request).await
    }

    /// GET /images/{name}/json
    /// Inspect an image
    /// Return low-level information about an image.
    pub async fn image_inspect(
        &self,
        name: &str,
    ) -> Result<super::super::models::ImageInspect, Error> {
        let mut request_url = url::Url::parse(self.config.get_base_path())?;

        let mut request_path = request_url.path().to_owned();
        if request_path.ends_with('/') {
            request_path.pop();
        }
        request_path.push_str("/images/{name}/json");
        request_path = request_path.replace("{name}", name);

        request_url.set_path(&request_path);

        let mut req_builder = self.config.req_builder("GET")?;

        let hyper_uri: hyper::Uri = request_url.as_str().parse()?;
        req_builder = req_builder.uri(hyper_uri);
        let request = req_builder.body(String::new())?;
        request::execute_request_json(&*self.config, request).await
    }

    /// POST /images/{name}/push
    /// Push Image
    /// Push an image to a container registry
    pub async fn image_push<'a>(
        &self,
        name: &str,
        params: Option<super::super::params::ImagePush<'a>>,
    ) -> Result<String, Error> {
        let mut request_url = url::Url::parse(self.config.get_base_path())?;

        let mut request_path = request_url.path().to_owned();
        if request_path.ends_with('/') {
            request_path.pop();
        }
        request_path.push_str("/images/{name}/push");
        request_path = request_path.replace("{name}", name);

        request_url.set_path(&request_path);

        let mut req_builder = self.config.req_builder("POST")?;

        if let Some(params) = params {
            let mut query_pairs = request_url.query_pairs_mut();
            if let Some(tag) = params.tag {
                query_pairs.append_pair("tag", &tag);
            }
            if let Some(all) = params.all {
                query_pairs.append_pair("all", &all.to_string());
            }
            if let Some(compress) = params.compress {
                query_pairs.append_pair("compress", &compress.to_string());
            }
            if let Some(destination) = params.destination {
                query_pairs.append_pair("destination", &destination);
            }

            if let Some(x_registry_auth) = params.x_registry_auth {
                req_builder = req_builder.header("X-Registry-Auth", x_registry_auth);
            }
        }

        let hyper_uri: hyper::Uri = request_url.as_str().parse()?;
        req_builder = req_builder.uri(hyper_uri);
        let request = req_builder.body(String::new())?;
        request::execute_request_json(&*self.config, request).await
    }

    /// POST /images/{name}/tag
    /// Tag an image
    /// Tag an image so that it becomes part of a repository.
    pub async fn image_tag<'a>(
        &self,
        name: &str,
        params: Option<super::super::params::ImageTag<'a>>,
    ) -> Result<(), Error> {
        let mut request_url = url::Url::parse(self.config.get_base_path())?;

        let mut request_path = request_url.path().to_owned();
        if request_path.ends_with('/') {
            request_path.pop();
        }
        request_path.push_str("/images/{name}/tag");
        request_path = request_path.replace("{name}", name);

        request_url.set_path(&request_path);

        let mut req_builder = self.config.req_builder("POST")?;

        if let Some(params) = params {
            let mut query_pairs = request_url.query_pairs_mut();
            if let Some(repo) = params.repo {
                query_pairs.append_pair("repo", &repo);
            }
            if let Some(tag) = params.tag {
                query_pairs.append_pair("tag", &tag);
            }
        }

        let hyper_uri: hyper::Uri = request_url.as_str().parse()?;
        req_builder = req_builder.uri(hyper_uri);
        let request = req_builder.body(String::new())?;
        request::execute_request_unit(&*self.config, request).await
    }

    /// POST /images/create
    /// Create an image
    /// Create an image by either pulling it from a registry or importing it.
    pub async fn image_create<'a>(
        &self,
        params: Option<super::super::params::ImageCreate<'a>>,
        input_image: String,
    ) -> Result<String, Error> {
        let mut request_url = url::Url::parse(self.config.get_base_path())?;

        let mut request_path = request_url.path().to_owned();
        if request_path.ends_with('/') {
            request_path.pop();
        }
        request_path.push_str("/images/create");

        request_url.set_path(&request_path);

        let mut req_builder = self.config.req_builder("POST")?;

        if let Some(params) = params {
            let mut query_pairs = request_url.query_pairs_mut();
            if let Some(from_image) = params.from_image {
                query_pairs.append_pair("fromImage", &from_image);
            }
            if let Some(from_src) = params.from_src {
                query_pairs.append_pair("fromSrc", &from_src);
            }
            if let Some(repo) = params.repo {
                query_pairs.append_pair("repo", &repo);
            }
            if let Some(tag) = params.tag {
                query_pairs.append_pair("tag", &tag);
            }
            if let Some(message) = params.message {
                query_pairs.append_pair("message", &message);
            }
            if let Some(platform) = params.platform {
                query_pairs.append_pair("platform", &platform);
            }

            if let Some(x_registry_auth) = params.x_registry_auth {
                req_builder = req_builder.header("X-Registry-Auth", x_registry_auth);
            }
        }

        let hyper_uri: hyper::Uri = request_url.as_str().parse()?;
        req_builder = req_builder.uri(hyper_uri);
        let body = serde_json::to_string(&input_image)?;
        req_builder = req_builder.header(hyper::header::CONTENT_TYPE, "application/json");
        req_builder = req_builder.header(hyper::header::CONTENT_LENGTH, body.len());
        let request = req_builder.body(body)?;
        request::execute_request_json(&*self.config, request).await
    }

    /// GET /images/get
    /// Export several images
    /// Get a tarball containing all images and metadata for several image repositories
    pub async fn image_get_all<'a>(
        &self,
        params: Option<super::super::params::ImageGetAll<'a>>,
    ) -> Result<String, Error> {
        let mut request_url = url::Url::parse(self.config.get_base_path())?;

        let mut request_path = request_url.path().to_owned();
        if request_path.ends_with('/') {
            request_path.pop();
        }
        request_path.push_str("/images/get");

        request_url.set_path(&request_path);

        let mut req_builder = self.config.req_builder("GET")?;

        if let Some(params) = params {
            let mut query_pairs = request_url.query_pairs_mut();
            query_pairs.append_pair("names", &params.names);
        }

        let hyper_uri: hyper::Uri = request_url.as_str().parse()?;
        req_builder = req_builder.uri(hyper_uri);
        let request = req_builder.body(String::new())?;
        request::execute_request_json(&*self.config, request).await
    }

    /// GET /images/json
    /// List Images
    /// Returns a list of images on the server. Note that it uses a different, smaller representation of an image than inspecting a single image.
    pub async fn image_list<'a>(
        &self,
        params: Option<super::super::params::ImageList<'a>>,
    ) -> Result<Vec<super::super::models::Summary>, Error> {
        let mut request_url = url::Url::parse(self.config.get_base_path())?;

        let mut request_path = request_url.path().to_owned();
        if request_path.ends_with('/') {
            request_path.pop();
        }
        request_path.push_str("/images/json");

        request_url.set_path(&request_path);

        let mut req_builder = self.config.req_builder("GET")?;

        if let Some(params) = params {
            let mut query_pairs = request_url.query_pairs_mut();
            if let Some(all) = params.all {
                query_pairs.append_pair("all", &all.to_string());
            }
            if let Some(filters) = params.filters {
                query_pairs.append_pair("filters", &filters);
            }
            if let Some(digests) = params.digests {
                query_pairs.append_pair("digests", &digests.to_string());
            }
        }

        let hyper_uri: hyper::Uri = request_url.as_str().parse()?;
        req_builder = req_builder.uri(hyper_uri);
        let request = req_builder.body(String::new())?;
        request::execute_request_json(&*self.config, request).await
    }

    /// POST /images/load
    /// Import image
    /// Load a set of images and tags into a repository.
    pub async fn image_load(
        &self,
        params: Option<super::super::params::ImageLoad>,
        request: String,
    ) -> Result<(), Error> {
        let mut request_url = url::Url::parse(self.config.get_base_path())?;

        let mut request_path = request_url.path().to_owned();
        if request_path.ends_with('/') {
            request_path.pop();
        }
        request_path.push_str("/images/load");

        request_url.set_path(&request_path);

        let mut req_builder = self.config.req_builder("POST")?;

        if let Some(params) = params {
            let mut query_pairs = request_url.query_pairs_mut();
            if let Some(quiet) = params.quiet {
                query_pairs.append_pair("quiet", &quiet.to_string());
            }
        }

        let hyper_uri: hyper::Uri = request_url.as_str().parse()?;
        req_builder = req_builder.uri(hyper_uri);
        let body = serde_json::to_string(&request)?;
        req_builder = req_builder.header(hyper::header::CONTENT_TYPE, "application/json");
        req_builder = req_builder.header(hyper::header::CONTENT_LENGTH, body.len());
        let request = req_builder.body(body)?;
        request::execute_request_unit(&*self.config, request).await
    }

    /// POST /images/prune
    /// Prune unused images
    /// Remove images from local storage that are not being used by a container
    pub async fn image_prune<'a>(
        &self,
        params: Option<super::super::params::ImagePrune<'a>>,
    ) -> Result<Vec<super::super::models::ImageDeleteResponseItems>, Error> {
        let mut request_url = url::Url::parse(self.config.get_base_path())?;

        let mut request_path = request_url.path().to_owned();
        if request_path.ends_with('/') {
            request_path.pop();
        }
        request_path.push_str("/images/prune");

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

    /// GET /images/search
    /// Search images
    /// Search registries for an image
    pub async fn image_search<'a>(
        &self,
        params: Option<super::super::params::ImageSearch<'a>>,
    ) -> Result<super::super::models::RegistrySearchResponse, Error> {
        let mut request_url = url::Url::parse(self.config.get_base_path())?;

        let mut request_path = request_url.path().to_owned();
        if request_path.ends_with('/') {
            request_path.pop();
        }
        request_path.push_str("/images/search");

        request_url.set_path(&request_path);

        let mut req_builder = self.config.req_builder("GET")?;

        if let Some(params) = params {
            let mut query_pairs = request_url.query_pairs_mut();
            if let Some(term) = params.term {
                query_pairs.append_pair("term", &term);
            }
            if let Some(limit) = params.limit {
                query_pairs.append_pair("limit", &limit.to_string());
            }
            if let Some(filters) = params.filters {
                query_pairs.append_pair("filters", &filters);
            }
            if let Some(tls_verify) = params.tls_verify {
                query_pairs.append_pair("tlsVerify", &tls_verify.to_string());
            }
            if let Some(list_tags) = params.list_tags {
                query_pairs.append_pair("listTags", &list_tags.to_string());
            }
        }

        let hyper_uri: hyper::Uri = request_url.as_str().parse()?;
        req_builder = req_builder.uri(hyper_uri);
        let request = req_builder.body(String::new())?;
        request::execute_request_json(&*self.config, request).await
    }
}

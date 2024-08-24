use crate::api_common::config::HasConfig;
use crate::api_common::request;
use crate::api_common::Error;
use http::request::Builder;
use std::future::Future;
use std::pin::Pin;
pub trait ImagesCompat: HasConfig + Send + Sync {
    /// POST /build
    ///
    /// Create image
    ///
    /// Build an image from the given Dockerfile(s)
    fn image_build<'a>(
        &'a self,
        params: Option<crate::v5::params::ImageBuild<'a>>,
        input_stream: String,
    ) -> Pin<Box<dyn Future<Output = Result<crate::v5::models::ImageBuild200, Error>> + Send + 'a>>
    {
        Box::pin(request::execute_request_json(
            self.get_config(),
            move |mut req_builder: Builder| {
                req_builder = req_builder.method("POST");
                let mut request_url = url::Url::parse(self.get_config().get_base_path())?;
                let mut request_path = request_url.path().to_owned();
                if request_path.ends_with('/') {
                    request_path.pop();
                }
                request_path.push_str("/build");
                request_url.set_path(&request_path);
                if let Some(params) = &params {
                    let mut query_pairs = request_url.query_pairs_mut();
                    if let Some(dockerfile) = params.dockerfile {
                        query_pairs.append_pair("dockerfile", dockerfile);
                    }
                    if let Some(t) = params.t {
                        query_pairs.append_pair("t", t);
                    }
                    if let Some(extrahosts) = params.extrahosts {
                        query_pairs.append_pair("extrahosts", extrahosts);
                    }
                    if let Some(remote) = params.remote {
                        query_pairs.append_pair("remote", remote);
                    }
                    if let Some(q) = params.q {
                        query_pairs.append_pair("q", &q.to_string());
                    }
                    if let Some(nocache) = params.nocache {
                        query_pairs.append_pair("nocache", &nocache.to_string());
                    }
                    if let Some(cachefrom) = params.cachefrom {
                        query_pairs.append_pair("cachefrom", cachefrom);
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
                        query_pairs.append_pair("cpusetcpus", cpusetcpus);
                    }
                    if let Some(cpuperiod) = params.cpuperiod {
                        query_pairs.append_pair("cpuperiod", &cpuperiod.to_string());
                    }
                    if let Some(cpuquota) = params.cpuquota {
                        query_pairs.append_pair("cpuquota", &cpuquota.to_string());
                    }
                    if let Some(buildargs) = params.buildargs {
                        query_pairs.append_pair("buildargs", buildargs);
                    }
                    if let Some(shmsize) = params.shmsize {
                        query_pairs.append_pair("shmsize", &shmsize.to_string());
                    }
                    if let Some(squash) = params.squash {
                        query_pairs.append_pair("squash", &squash.to_string());
                    }
                    if let Some(labels) = params.labels {
                        query_pairs.append_pair("labels", labels);
                    }
                    if let Some(networkmode) = params.networkmode {
                        query_pairs.append_pair("networkmode", networkmode);
                    }
                    if let Some(platform) = params.platform {
                        query_pairs.append_pair("platform", platform);
                    }
                    if let Some(target) = params.target {
                        query_pairs.append_pair("target", target);
                    }
                    if let Some(outputs) = params.outputs {
                        query_pairs.append_pair("outputs", outputs);
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
                Ok(req_builder.body(body)?)
            },
        ))
    }
    /// DELETE /images/{name}
    ///
    /// Remove Image
    ///
    /// Delete an image from local storage
    fn image_delete<'a>(
        &'a self,
        name: &'a str,
        params: Option<crate::v5::params::ImageDelete>,
    ) -> Pin<
        Box<
            dyn Future<Output = Result<Vec<crate::v5::models::ImageDeleteResponseItems>, Error>>
                + Send
                + 'a,
        >,
    > {
        Box::pin(request::execute_request_json(
            self.get_config(),
            move |mut req_builder: Builder| {
                req_builder = req_builder.method("DELETE");
                let mut request_url = url::Url::parse(self.get_config().get_base_path())?;
                let mut request_path = request_url.path().to_owned();
                if request_path.ends_with('/') {
                    request_path.pop();
                }
                request_path.push_str("/images/{name}");
                request_path = request_path.replace("{name}", name);
                request_url.set_path(&request_path);
                if let Some(params) = &params {
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
                Ok(req_builder.body(String::new())?)
            },
        ))
    }
    /// GET /images/{name}/get
    ///
    /// Export an image
    ///
    /// Export an image in tarball format
    fn image_get<'a>(
        &'a self,
        name: &'a str,
    ) -> Pin<Box<dyn futures::stream::Stream<Item = Result<bytes::Bytes, Error>> + Send + 'a>> {
        request::execute_request_stream(self.get_config(), move |mut req_builder: Builder| {
            req_builder = req_builder.method("GET");
            let mut request_url = url::Url::parse(self.get_config().get_base_path())?;
            let mut request_path = request_url.path().to_owned();
            if request_path.ends_with('/') {
                request_path.pop();
            }
            request_path.push_str("/images/{name}/get");
            request_path = request_path.replace("{name}", name);
            request_url.set_path(&request_path);
            let hyper_uri: hyper::Uri = request_url.as_str().parse()?;
            req_builder = req_builder.uri(hyper_uri);
            Ok(req_builder.body(String::new())?)
        })
    }
    /// GET /images/{name}/history
    ///
    /// History of an image
    ///
    /// Return parent layers of an image.
    fn image_history<'a>(
        &'a self,
        name: &'a str,
    ) -> Pin<Box<dyn Future<Output = Result<crate::v5::models::HistoryResponse, Error>> + Send + 'a>>
    {
        Box::pin(request::execute_request_json(
            self.get_config(),
            move |mut req_builder: Builder| {
                req_builder = req_builder.method("GET");
                let mut request_url = url::Url::parse(self.get_config().get_base_path())?;
                let mut request_path = request_url.path().to_owned();
                if request_path.ends_with('/') {
                    request_path.pop();
                }
                request_path.push_str("/images/{name}/history");
                request_path = request_path.replace("{name}", name);
                request_url.set_path(&request_path);
                let hyper_uri: hyper::Uri = request_url.as_str().parse()?;
                req_builder = req_builder.uri(hyper_uri);
                Ok(req_builder.body(String::new())?)
            },
        ))
    }
    /// GET /images/{name}/json
    ///
    /// Inspect an image
    ///
    /// Return low-level information about an image.
    fn image_inspect<'a>(
        &'a self,
        name: &'a str,
    ) -> Pin<Box<dyn Future<Output = Result<crate::v5::models::ImageInspect, Error>> + Send + 'a>>
    {
        Box::pin(request::execute_request_json(
            self.get_config(),
            move |mut req_builder: Builder| {
                req_builder = req_builder.method("GET");
                let mut request_url = url::Url::parse(self.get_config().get_base_path())?;
                let mut request_path = request_url.path().to_owned();
                if request_path.ends_with('/') {
                    request_path.pop();
                }
                request_path.push_str("/images/{name}/json");
                request_path = request_path.replace("{name}", name);
                request_url.set_path(&request_path);
                let hyper_uri: hyper::Uri = request_url.as_str().parse()?;
                req_builder = req_builder.uri(hyper_uri);
                Ok(req_builder.body(String::new())?)
            },
        ))
    }
    /// POST /images/{name}/push
    ///
    /// Push Image
    ///
    /// Push an image to a container registry
    fn image_push<'a>(
        &'a self,
        name: &'a str,
        params: Option<crate::v5::params::ImagePush<'a>>,
    ) -> Pin<Box<dyn Future<Output = Result<String, Error>> + Send + 'a>> {
        Box::pin(request::execute_request_json(
            self.get_config(),
            move |mut req_builder: Builder| {
                req_builder = req_builder.method("POST");
                let mut request_url = url::Url::parse(self.get_config().get_base_path())?;
                let mut request_path = request_url.path().to_owned();
                if request_path.ends_with('/') {
                    request_path.pop();
                }
                request_path.push_str("/images/{name}/push");
                request_path = request_path.replace("{name}", name);
                request_url.set_path(&request_path);
                if let Some(params) = &params {
                    let mut query_pairs = request_url.query_pairs_mut();
                    if let Some(tag) = params.tag {
                        query_pairs.append_pair("tag", tag);
                    }
                    if let Some(all) = params.all {
                        query_pairs.append_pair("all", &all.to_string());
                    }
                    if let Some(compress) = params.compress {
                        query_pairs.append_pair("compress", &compress.to_string());
                    }
                    if let Some(destination) = params.destination {
                        query_pairs.append_pair("destination", destination);
                    }
                    if let Some(x_registry_auth) = params.x_registry_auth {
                        req_builder = req_builder.header("X-Registry-Auth", x_registry_auth);
                    }
                }
                let hyper_uri: hyper::Uri = request_url.as_str().parse()?;
                req_builder = req_builder.uri(hyper_uri);
                Ok(req_builder.body(String::new())?)
            },
        ))
    }
    /// POST /images/{name}/tag
    ///
    /// Tag an image
    ///
    /// Tag an image so that it becomes part of a repository.
    fn image_tag<'a>(
        &'a self,
        name: &'a str,
        params: Option<crate::v5::params::ImageTag<'a>>,
    ) -> Pin<Box<dyn Future<Output = Result<(), Error>> + Send + 'a>> {
        Box::pin(request::execute_request_unit(
            self.get_config(),
            move |mut req_builder: Builder| {
                req_builder = req_builder.method("POST");
                let mut request_url = url::Url::parse(self.get_config().get_base_path())?;
                let mut request_path = request_url.path().to_owned();
                if request_path.ends_with('/') {
                    request_path.pop();
                }
                request_path.push_str("/images/{name}/tag");
                request_path = request_path.replace("{name}", name);
                request_url.set_path(&request_path);
                if let Some(params) = &params {
                    let mut query_pairs = request_url.query_pairs_mut();
                    if let Some(repo) = params.repo {
                        query_pairs.append_pair("repo", repo);
                    }
                    if let Some(tag) = params.tag {
                        query_pairs.append_pair("tag", tag);
                    }
                }
                let hyper_uri: hyper::Uri = request_url.as_str().parse()?;
                req_builder = req_builder.uri(hyper_uri);
                Ok(req_builder.body(String::new())?)
            },
        ))
    }
    /// POST /images/create
    ///
    /// Create an image
    ///
    /// Create an image by either pulling it from a registry or importing it.
    fn image_create<'a>(
        &'a self,
        params: Option<crate::v5::params::ImageCreate<'a>>,
        input_image: String,
    ) -> Pin<Box<dyn Future<Output = Result<String, Error>> + Send + 'a>> {
        Box::pin(request::execute_request_json(
            self.get_config(),
            move |mut req_builder: Builder| {
                req_builder = req_builder.method("POST");
                let mut request_url = url::Url::parse(self.get_config().get_base_path())?;
                let mut request_path = request_url.path().to_owned();
                if request_path.ends_with('/') {
                    request_path.pop();
                }
                request_path.push_str("/images/create");
                request_url.set_path(&request_path);
                if let Some(params) = &params {
                    let mut query_pairs = request_url.query_pairs_mut();
                    if let Some(from_image) = params.from_image {
                        query_pairs.append_pair("fromImage", from_image);
                    }
                    if let Some(from_src) = params.from_src {
                        query_pairs.append_pair("fromSrc", from_src);
                    }
                    if let Some(repo) = params.repo {
                        query_pairs.append_pair("repo", repo);
                    }
                    if let Some(tag) = params.tag {
                        query_pairs.append_pair("tag", tag);
                    }
                    if let Some(message) = params.message {
                        query_pairs.append_pair("message", message);
                    }
                    if let Some(platform) = params.platform {
                        query_pairs.append_pair("platform", platform);
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
                Ok(req_builder.body(body)?)
            },
        ))
    }
    /// GET /images/get
    ///
    /// Export several images
    ///
    /// Get a tarball containing all images and metadata for several image repositories
    fn image_get_all<'a>(
        &'a self,
        params: Option<crate::v5::params::ImageGetAll<'a>>,
    ) -> Pin<Box<dyn Future<Output = Result<String, Error>> + Send + 'a>> {
        Box::pin(request::execute_request_json(
            self.get_config(),
            move |mut req_builder: Builder| {
                req_builder = req_builder.method("GET");
                let mut request_url = url::Url::parse(self.get_config().get_base_path())?;
                let mut request_path = request_url.path().to_owned();
                if request_path.ends_with('/') {
                    request_path.pop();
                }
                request_path.push_str("/images/get");
                request_url.set_path(&request_path);
                if let Some(params) = &params {
                    let mut query_pairs = request_url.query_pairs_mut();
                    query_pairs.append_pair("names", params.names);
                }
                let hyper_uri: hyper::Uri = request_url.as_str().parse()?;
                req_builder = req_builder.uri(hyper_uri);
                Ok(req_builder.body(String::new())?)
            },
        ))
    }
    /// GET /images/json
    ///
    /// List Images
    ///
    /// Returns a list of images on the server. Note that it uses a different, smaller representation of an image than inspecting a single image.
    fn image_list<'a>(
        &'a self,
        params: Option<crate::v5::params::ImageList<'a>>,
    ) -> Pin<Box<dyn Future<Output = Result<Vec<crate::v5::models::Summary>, Error>> + Send + 'a>>
    {
        Box::pin(request::execute_request_json(
            self.get_config(),
            move |mut req_builder: Builder| {
                req_builder = req_builder.method("GET");
                let mut request_url = url::Url::parse(self.get_config().get_base_path())?;
                let mut request_path = request_url.path().to_owned();
                if request_path.ends_with('/') {
                    request_path.pop();
                }
                request_path.push_str("/images/json");
                request_url.set_path(&request_path);
                if let Some(params) = &params {
                    let mut query_pairs = request_url.query_pairs_mut();
                    if let Some(all) = params.all {
                        query_pairs.append_pair("all", &all.to_string());
                    }
                    if let Some(filters) = params.filters {
                        query_pairs.append_pair("filters", filters);
                    }
                    if let Some(digests) = params.digests {
                        query_pairs.append_pair("digests", &digests.to_string());
                    }
                }
                let hyper_uri: hyper::Uri = request_url.as_str().parse()?;
                req_builder = req_builder.uri(hyper_uri);
                Ok(req_builder.body(String::new())?)
            },
        ))
    }
    /// POST /images/load
    ///
    /// Import image
    ///
    /// Load a set of images and tags into a repository.
    fn image_load<'a>(
        &'a self,
        params: Option<crate::v5::params::ImageLoad>,
        request: String,
    ) -> Pin<Box<dyn Future<Output = Result<(), Error>> + Send + 'a>> {
        Box::pin(request::execute_request_unit(
            self.get_config(),
            move |mut req_builder: Builder| {
                req_builder = req_builder.method("POST");
                let mut request_url = url::Url::parse(self.get_config().get_base_path())?;
                let mut request_path = request_url.path().to_owned();
                if request_path.ends_with('/') {
                    request_path.pop();
                }
                request_path.push_str("/images/load");
                request_url.set_path(&request_path);
                if let Some(params) = &params {
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
                Ok(req_builder.body(body)?)
            },
        ))
    }
    /// POST /images/prune
    ///
    /// Prune unused images
    ///
    /// Remove images from local storage that are not being used by a container
    fn image_prune<'a>(
        &'a self,
        params: Option<crate::v5::params::ImagePrune<'a>>,
    ) -> Pin<
        Box<
            dyn Future<Output = Result<Vec<crate::v5::models::ImageDeleteResponseItems>, Error>>
                + Send
                + 'a,
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
                request_path.push_str("/images/prune");
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
    /// GET /images/search
    ///
    /// Search images
    ///
    /// Search registries for an image
    fn image_search<'a>(
        &'a self,
        params: Option<crate::v5::params::ImageSearch<'a>>,
    ) -> Pin<
        Box<
            dyn Future<Output = Result<crate::v5::models::RegistrySearchResponse, Error>>
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
                request_path.push_str("/images/search");
                request_url.set_path(&request_path);
                if let Some(params) = &params {
                    let mut query_pairs = request_url.query_pairs_mut();
                    if let Some(term) = params.term {
                        query_pairs.append_pair("term", term);
                    }
                    if let Some(limit) = params.limit {
                        query_pairs.append_pair("limit", &limit.to_string());
                    }
                    if let Some(filters) = params.filters {
                        query_pairs.append_pair("filters", filters);
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
                Ok(req_builder.body(String::new())?)
            },
        ))
    }
}

use super::super::config::HasConfig;
use super::super::request;
use super::super::Error;
#[async_trait::async_trait]
pub trait Images: HasConfig + Send + Sync {
    /// POST /libpod/build
    /// Create image
    /// Build an image from the given Dockerfile(s)
    async fn image_build_libpod<'a>(
        &self,
        params: Option<super::super::params::ImageBuildLibpod<'a>>,
    ) -> Result<super::super::models::ImageBuildLibpod200, Error> {
        let mut request_url = url::Url::parse(self.get_config().get_base_path())?;
        let mut request_path = request_url.path().to_owned();
        if request_path.ends_with('/') {
            request_path.pop();
        }
        request_path.push_str("/libpod/build");
        request_url.set_path(&request_path);
        let mut req_builder = self.get_config().req_builder("POST")?;
        if let Some(params) = params {
            let mut query_pairs = request_url.query_pairs_mut();
            if let Some(dockerfile) = params.dockerfile {
                query_pairs.append_pair("dockerfile", dockerfile);
            }
            if let Some(t) = params.t {
                query_pairs.append_pair("t", t);
            }
            if let Some(allplatforms) = params.allplatforms {
                query_pairs.append_pair("allplatforms", &allplatforms.to_string());
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
            if let Some(layer_label) = params.layer_label {
                query_pairs.append_pair(
                    "layerLabel",
                    &layer_label
                        .iter()
                        .map(|e| e.to_string())
                        .collect::<Vec<_>>()
                        .join(","),
                );
            }
            if let Some(layers) = params.layers {
                query_pairs.append_pair("layers", &layers.to_string());
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
            if let Some(httpproxy) = params.httpproxy {
                query_pairs.append_pair("httpproxy", &httpproxy.to_string());
            }
            if let Some(unsetenv) = params.unsetenv {
                query_pairs.append_pair(
                    "unsetenv",
                    &unsetenv
                        .iter()
                        .map(|e| e.to_string())
                        .collect::<Vec<_>>()
                        .join(","),
                );
            }
            if let Some(unsetlabel) = params.unsetlabel {
                query_pairs.append_pair(
                    "unsetlabel",
                    &unsetlabel
                        .iter()
                        .map(|e| e.to_string())
                        .collect::<Vec<_>>()
                        .join(","),
                );
            }
            if let Some(volume) = params.volume {
                query_pairs.append_pair(
                    "volume",
                    &volume
                        .iter()
                        .map(|e| e.to_string())
                        .collect::<Vec<_>>()
                        .join(","),
                );
            }
        }
        let hyper_uri: hyper::Uri = request_url.as_str().parse()?;
        req_builder = req_builder.uri(hyper_uri);
        let request = req_builder.body(String::new())?;
        request::execute_request_json(self.get_config(), request).await
    }
    /// DELETE /libpod/images/{name}
    /// Remove an image from the local storage.
    /// Remove an image from the local storage.
    async fn image_delete_libpod(
        &self,
        name: &str,
        params: Option<super::super::params::ImageDeleteLibpod>,
    ) -> Result<super::super::models::LibpodImagesRemoveReport, Error> {
        let mut request_url = url::Url::parse(self.get_config().get_base_path())?;
        let mut request_path = request_url.path().to_owned();
        if request_path.ends_with('/') {
            request_path.pop();
        }
        request_path.push_str("/libpod/images/{name}");
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
    }
    /// GET /libpod/images/{name}/changes
    /// Report on changes to images's filesystem; adds, deletes or modifications.
    /// Returns which files in an image's filesystem have been added, deleted, or modified. The Kind of modification can be one of:
    ///
    /// 0: Modified
    /// 1: Added
    /// 2: Deleted
    async fn image_changes_libpod<'a>(
        &self,
        name: &str,
        params: Option<super::super::params::ImageChangesLibpod<'a>>,
    ) -> Result<(), Error> {
        let mut request_url = url::Url::parse(self.get_config().get_base_path())?;
        let mut request_path = request_url.path().to_owned();
        if request_path.ends_with('/') {
            request_path.pop();
        }
        request_path.push_str("/libpod/images/{name}/changes");
        request_path = request_path.replace("{name}", name);
        request_url.set_path(&request_path);
        let mut req_builder = self.get_config().req_builder("GET")?;
        if let Some(params) = params {
            let mut query_pairs = request_url.query_pairs_mut();
            if let Some(parent) = params.parent {
                query_pairs.append_pair("parent", parent);
            }
            if let Some(diff_type) = params.diff_type {
                query_pairs.append_pair("diffType", diff_type);
            }
        }
        let hyper_uri: hyper::Uri = request_url.as_str().parse()?;
        req_builder = req_builder.uri(hyper_uri);
        let request = req_builder.body(String::new())?;
        request::execute_request_unit(self.get_config(), request).await
    }
    /// GET /libpod/images/{name}/exists
    /// Image exists
    /// Check if image exists in local store
    async fn image_exists_libpod(&self, name: &str) -> Result<(), Error> {
        let mut request_url = url::Url::parse(self.get_config().get_base_path())?;
        let mut request_path = request_url.path().to_owned();
        if request_path.ends_with('/') {
            request_path.pop();
        }
        request_path.push_str("/libpod/images/{name}/exists");
        request_path = request_path.replace("{name}", name);
        request_url.set_path(&request_path);
        let mut req_builder = self.get_config().req_builder("GET")?;
        let hyper_uri: hyper::Uri = request_url.as_str().parse()?;
        req_builder = req_builder.uri(hyper_uri);
        let request = req_builder.body(String::new())?;
        request::execute_request_unit(self.get_config(), request).await
    }
    /// GET /libpod/images/{name}/get
    /// Export an image
    /// Export an image
    async fn image_get_libpod<'a>(
        &self,
        name: &str,
        params: Option<super::super::params::ImageGetLibpod<'a>>,
    ) -> Result<String, Error> {
        let mut request_url = url::Url::parse(self.get_config().get_base_path())?;
        let mut request_path = request_url.path().to_owned();
        if request_path.ends_with('/') {
            request_path.pop();
        }
        request_path.push_str("/libpod/images/{name}/get");
        request_path = request_path.replace("{name}", name);
        request_url.set_path(&request_path);
        let mut req_builder = self.get_config().req_builder("GET")?;
        if let Some(params) = params {
            let mut query_pairs = request_url.query_pairs_mut();
            if let Some(format) = params.format {
                query_pairs.append_pair("format", format);
            }
            if let Some(compress) = params.compress {
                query_pairs.append_pair("compress", &compress.to_string());
            }
        }
        let hyper_uri: hyper::Uri = request_url.as_str().parse()?;
        req_builder = req_builder.uri(hyper_uri);
        let request = req_builder.body(String::new())?;
        request::execute_request_json(self.get_config(), request).await
    }
    /// GET /libpod/images/{name}/history
    /// History of an image
    /// Return parent layers of an image.
    async fn image_history_libpod(
        &self,
        name: &str,
    ) -> Result<super::super::models::HistoryResponse, Error> {
        let mut request_url = url::Url::parse(self.get_config().get_base_path())?;
        let mut request_path = request_url.path().to_owned();
        if request_path.ends_with('/') {
            request_path.pop();
        }
        request_path.push_str("/libpod/images/{name}/history");
        request_path = request_path.replace("{name}", name);
        request_url.set_path(&request_path);
        let mut req_builder = self.get_config().req_builder("GET")?;
        let hyper_uri: hyper::Uri = request_url.as_str().parse()?;
        req_builder = req_builder.uri(hyper_uri);
        let request = req_builder.body(String::new())?;
        request::execute_request_json(self.get_config(), request).await
    }
    /// GET /libpod/images/{name}/json
    /// Inspect an image
    /// Obtain low-level information about an image
    async fn image_inspect_libpod(
        &self,
        name: &str,
    ) -> Result<super::super::models::ImageData, Error> {
        let mut request_url = url::Url::parse(self.get_config().get_base_path())?;
        let mut request_path = request_url.path().to_owned();
        if request_path.ends_with('/') {
            request_path.pop();
        }
        request_path.push_str("/libpod/images/{name}/json");
        request_path = request_path.replace("{name}", name);
        request_url.set_path(&request_path);
        let mut req_builder = self.get_config().req_builder("GET")?;
        let hyper_uri: hyper::Uri = request_url.as_str().parse()?;
        req_builder = req_builder.uri(hyper_uri);
        let request = req_builder.body(String::new())?;
        request::execute_request_json(self.get_config(), request).await
    }
    /// POST /libpod/images/{name}/push
    /// Push Image
    /// Push an image to a container registry
    async fn image_push_libpod<'a>(
        &self,
        name: &str,
        params: Option<super::super::params::ImagePushLibpod<'a>>,
    ) -> Result<String, Error> {
        let mut request_url = url::Url::parse(self.get_config().get_base_path())?;
        let mut request_path = request_url.path().to_owned();
        if request_path.ends_with('/') {
            request_path.pop();
        }
        request_path.push_str("/libpod/images/{name}/push");
        request_path = request_path.replace("{name}", name);
        request_url.set_path(&request_path);
        let mut req_builder = self.get_config().req_builder("POST")?;
        if let Some(params) = params {
            let mut query_pairs = request_url.query_pairs_mut();
            if let Some(destination) = params.destination {
                query_pairs.append_pair("destination", destination);
            }
            if let Some(force_compression_format) = params.force_compression_format {
                query_pairs.append_pair(
                    "forceCompressionFormat",
                    &force_compression_format.to_string(),
                );
            }
            if let Some(tls_verify) = params.tls_verify {
                query_pairs.append_pair("tlsVerify", &tls_verify.to_string());
            }
            if let Some(quiet) = params.quiet {
                query_pairs.append_pair("quiet", &quiet.to_string());
            }
            if let Some(x_registry_auth) = params.x_registry_auth {
                req_builder = req_builder.header("X-Registry-Auth", x_registry_auth);
            }
        }
        let hyper_uri: hyper::Uri = request_url.as_str().parse()?;
        req_builder = req_builder.uri(hyper_uri);
        let request = req_builder.body(String::new())?;
        request::execute_request_json(self.get_config(), request).await
    }
    /// GET /libpod/images/{name}/resolve
    /// Resolve an image (short) name
    /// Resolve the passed image name to a list of fully-qualified images referring to container registries.
    async fn image_resolve_libpod(&self, name: &str) -> Result<(), Error> {
        let mut request_url = url::Url::parse(self.get_config().get_base_path())?;
        let mut request_path = request_url.path().to_owned();
        if request_path.ends_with('/') {
            request_path.pop();
        }
        request_path.push_str("/libpod/images/{name}/resolve");
        request_path = request_path.replace("{name}", name);
        request_url.set_path(&request_path);
        let mut req_builder = self.get_config().req_builder("GET")?;
        let hyper_uri: hyper::Uri = request_url.as_str().parse()?;
        req_builder = req_builder.uri(hyper_uri);
        let request = req_builder.body(String::new())?;
        request::execute_request_unit(self.get_config(), request).await
    }
    /// POST /libpod/images/{name}/tag
    /// Tag an image
    /// Tag an image so that it becomes part of a repository.
    async fn image_tag_libpod<'a>(
        &self,
        name: &str,
        params: Option<super::super::params::ImageTagLibpod<'a>>,
    ) -> Result<(), Error> {
        let mut request_url = url::Url::parse(self.get_config().get_base_path())?;
        let mut request_path = request_url.path().to_owned();
        if request_path.ends_with('/') {
            request_path.pop();
        }
        request_path.push_str("/libpod/images/{name}/tag");
        request_path = request_path.replace("{name}", name);
        request_url.set_path(&request_path);
        let mut req_builder = self.get_config().req_builder("POST")?;
        if let Some(params) = params {
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
        let request = req_builder.body(String::new())?;
        request::execute_request_unit(self.get_config(), request).await
    }
    /// GET /libpod/images/{name}/tree
    /// Image tree
    /// Retrieve the image tree for the provided image name or ID
    async fn image_tree_libpod(
        &self,
        name: &str,
        params: Option<super::super::params::ImageTreeLibpod>,
    ) -> Result<super::super::models::ImageTreeReport, Error> {
        let mut request_url = url::Url::parse(self.get_config().get_base_path())?;
        let mut request_path = request_url.path().to_owned();
        if request_path.ends_with('/') {
            request_path.pop();
        }
        request_path.push_str("/libpod/images/{name}/tree");
        request_path = request_path.replace("{name}", name);
        request_url.set_path(&request_path);
        let mut req_builder = self.get_config().req_builder("GET")?;
        if let Some(params) = params {
            let mut query_pairs = request_url.query_pairs_mut();
            if let Some(whatrequires) = params.whatrequires {
                query_pairs.append_pair("whatrequires", &whatrequires.to_string());
            }
        }
        let hyper_uri: hyper::Uri = request_url.as_str().parse()?;
        req_builder = req_builder.uri(hyper_uri);
        let request = req_builder.body(String::new())?;
        request::execute_request_json(self.get_config(), request).await
    }
    /// POST /libpod/images/{name}/untag
    /// Untag an image
    /// Untag an image. If not repo and tag are specified, all tags are removed from the image.
    async fn image_untag_libpod<'a>(
        &self,
        name: &str,
        params: Option<super::super::params::ImageUntagLibpod<'a>>,
    ) -> Result<(), Error> {
        let mut request_url = url::Url::parse(self.get_config().get_base_path())?;
        let mut request_path = request_url.path().to_owned();
        if request_path.ends_with('/') {
            request_path.pop();
        }
        request_path.push_str("/libpod/images/{name}/untag");
        request_path = request_path.replace("{name}", name);
        request_url.set_path(&request_path);
        let mut req_builder = self.get_config().req_builder("POST")?;
        if let Some(params) = params {
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
        let request = req_builder.body(String::new())?;
        request::execute_request_unit(self.get_config(), request).await
    }
    /// GET /libpod/images/export
    /// Export multiple images
    /// Export multiple images into a single object. Only `docker-archive` is currently supported.
    async fn image_export_libpod<'a>(
        &self,
        params: Option<super::super::params::ImageExportLibpod<'a>>,
    ) -> Result<String, Error> {
        let mut request_url = url::Url::parse(self.get_config().get_base_path())?;
        let mut request_path = request_url.path().to_owned();
        if request_path.ends_with('/') {
            request_path.pop();
        }
        request_path.push_str("/libpod/images/export");
        request_url.set_path(&request_path);
        let mut req_builder = self.get_config().req_builder("GET")?;
        if let Some(params) = params {
            let mut query_pairs = request_url.query_pairs_mut();
            if let Some(format) = params.format {
                query_pairs.append_pair("format", format);
            }
            if let Some(references) = params.references {
                query_pairs.append_pair(
                    "references",
                    &references
                        .iter()
                        .map(|e| e.to_string())
                        .collect::<Vec<_>>()
                        .join(","),
                );
            }
            if let Some(compress) = params.compress {
                query_pairs.append_pair("compress", &compress.to_string());
            }
            if let Some(oci_accept_uncompressed_layers) = params.oci_accept_uncompressed_layers {
                query_pairs.append_pair(
                    "ociAcceptUncompressedLayers",
                    &oci_accept_uncompressed_layers.to_string(),
                );
            }
        }
        let hyper_uri: hyper::Uri = request_url.as_str().parse()?;
        req_builder = req_builder.uri(hyper_uri);
        let request = req_builder.body(String::new())?;
        request::execute_request_json(self.get_config(), request).await
    }
    /// POST /libpod/images/import
    /// Import image
    /// Import a previously exported tarball as an image.
    async fn image_import_libpod<'a>(
        &self,
        params: Option<super::super::params::ImageImportLibpod<'a>>,
        upload: String,
    ) -> Result<super::super::models::ImageImportReport, Error> {
        let mut request_url = url::Url::parse(self.get_config().get_base_path())?;
        let mut request_path = request_url.path().to_owned();
        if request_path.ends_with('/') {
            request_path.pop();
        }
        request_path.push_str("/libpod/images/import");
        request_url.set_path(&request_path);
        let mut req_builder = self.get_config().req_builder("POST")?;
        if let Some(params) = params {
            let mut query_pairs = request_url.query_pairs_mut();
            if let Some(changes) = params.changes {
                query_pairs.append_pair(
                    "changes",
                    &changes
                        .iter()
                        .map(|e| e.to_string())
                        .collect::<Vec<_>>()
                        .join(","),
                );
            }
            if let Some(message) = params.message {
                query_pairs.append_pair("message", message);
            }
            if let Some(reference) = params.reference {
                query_pairs.append_pair("reference", reference);
            }
            if let Some(url) = params.url {
                query_pairs.append_pair("url", url);
            }
            if let Some(content_type) = params.content_type {
                req_builder = req_builder.header("Content-Type", content_type);
            }
        }
        let hyper_uri: hyper::Uri = request_url.as_str().parse()?;
        req_builder = req_builder.uri(hyper_uri);
        let body = serde_json::to_string(&upload)?;
        req_builder = req_builder.header(hyper::header::CONTENT_TYPE, "application/json");
        req_builder = req_builder.header(hyper::header::CONTENT_LENGTH, body.len());
        let request = req_builder.body(body)?;
        request::execute_request_json(self.get_config(), request).await
    }
    /// GET /libpod/images/json
    /// List Images
    /// Returns a list of images on the server
    async fn image_list_libpod<'a>(
        &self,
        params: Option<super::super::params::ImageListLibpod<'a>>,
    ) -> Result<Vec<super::super::models::LibpodImageSummary>, Error> {
        let mut request_url = url::Url::parse(self.get_config().get_base_path())?;
        let mut request_path = request_url.path().to_owned();
        if request_path.ends_with('/') {
            request_path.pop();
        }
        request_path.push_str("/libpod/images/json");
        request_url.set_path(&request_path);
        let mut req_builder = self.get_config().req_builder("GET")?;
        if let Some(params) = params {
            let mut query_pairs = request_url.query_pairs_mut();
            if let Some(all) = params.all {
                query_pairs.append_pair("all", &all.to_string());
            }
            if let Some(filters) = params.filters {
                query_pairs.append_pair("filters", filters);
            }
        }
        let hyper_uri: hyper::Uri = request_url.as_str().parse()?;
        req_builder = req_builder.uri(hyper_uri);
        let request = req_builder.body(String::new())?;
        request::execute_request_json(self.get_config(), request).await
    }
    /// POST /libpod/images/load
    /// Load image
    /// Load an image (oci-archive or docker-archive) stream.
    async fn image_load_libpod(
        &self,
        upload: String,
    ) -> Result<super::super::models::ImageLoadReport, Error> {
        let mut request_url = url::Url::parse(self.get_config().get_base_path())?;
        let mut request_path = request_url.path().to_owned();
        if request_path.ends_with('/') {
            request_path.pop();
        }
        request_path.push_str("/libpod/images/load");
        request_url.set_path(&request_path);
        let mut req_builder = self.get_config().req_builder("POST")?;
        let hyper_uri: hyper::Uri = request_url.as_str().parse()?;
        req_builder = req_builder.uri(hyper_uri);
        let body = serde_json::to_string(&upload)?;
        req_builder = req_builder.header(hyper::header::CONTENT_TYPE, "application/json");
        req_builder = req_builder.header(hyper::header::CONTENT_LENGTH, body.len());
        let request = req_builder.body(body)?;
        request::execute_request_json(self.get_config(), request).await
    }
    /// POST /libpod/images/prune
    /// Prune unused images
    /// Remove images that are not being used by a container
    async fn image_prune_libpod<'a>(
        &self,
        params: Option<super::super::params::ImagePruneLibpod<'a>>,
    ) -> Result<Vec<super::super::models::PruneReport>, Error> {
        let mut request_url = url::Url::parse(self.get_config().get_base_path())?;
        let mut request_path = request_url.path().to_owned();
        if request_path.ends_with('/') {
            request_path.pop();
        }
        request_path.push_str("/libpod/images/prune");
        request_url.set_path(&request_path);
        let mut req_builder = self.get_config().req_builder("POST")?;
        if let Some(params) = params {
            let mut query_pairs = request_url.query_pairs_mut();
            if let Some(all) = params.all {
                query_pairs.append_pair("all", &all.to_string());
            }
            if let Some(external) = params.external {
                query_pairs.append_pair("external", &external.to_string());
            }
            if let Some(filters) = params.filters {
                query_pairs.append_pair("filters", filters);
            }
        }
        let hyper_uri: hyper::Uri = request_url.as_str().parse()?;
        req_builder = req_builder.uri(hyper_uri);
        let request = req_builder.body(String::new())?;
        request::execute_request_json(self.get_config(), request).await
    }
    /// POST /libpod/images/pull
    /// Pull images
    /// Pull one or more images from a container registry.
    async fn image_pull_libpod<'a>(
        &self,
        params: Option<super::super::params::ImagePullLibpod<'a>>,
    ) -> Result<super::super::models::LibpodImagesPullReport, Error> {
        let mut request_url = url::Url::parse(self.get_config().get_base_path())?;
        let mut request_path = request_url.path().to_owned();
        if request_path.ends_with('/') {
            request_path.pop();
        }
        request_path.push_str("/libpod/images/pull");
        request_url.set_path(&request_path);
        let mut req_builder = self.get_config().req_builder("POST")?;
        if let Some(params) = params {
            let mut query_pairs = request_url.query_pairs_mut();
            if let Some(reference) = params.reference {
                query_pairs.append_pair("reference", reference);
            }
            if let Some(quiet) = params.quiet.or(Some(true)) {
                query_pairs.append_pair("quiet", &quiet.to_string());
            }
            if let Some(compat_mode) = params.compat_mode {
                query_pairs.append_pair("compatMode", &compat_mode.to_string());
            }
            if let Some(arch) = params.arch {
                query_pairs.append_pair("Arch", arch);
            }
            if let Some(os) = params.os {
                query_pairs.append_pair("OS", os);
            }
            if let Some(variant) = params.variant {
                query_pairs.append_pair("Variant", variant);
            }
            if let Some(policy) = params.policy {
                query_pairs.append_pair("policy", policy);
            }
            if let Some(tls_verify) = params.tls_verify {
                query_pairs.append_pair("tlsVerify", &tls_verify.to_string());
            }
            if let Some(all_tags) = params.all_tags {
                query_pairs.append_pair("allTags", &all_tags.to_string());
            }
            if let Some(x_registry_auth) = params.x_registry_auth {
                req_builder = req_builder.header("X-Registry-Auth", x_registry_auth);
            }
        }
        let hyper_uri: hyper::Uri = request_url.as_str().parse()?;
        req_builder = req_builder.uri(hyper_uri);
        let request = req_builder.body(String::new())?;
        request::execute_request_json(self.get_config(), request).await
    }
    /// DELETE /libpod/images/remove
    /// Remove one or more images from the storage.
    /// Remove one or more images from the storage.
    async fn image_delete_all_libpod<'a>(
        &self,
        params: Option<super::super::params::ImageDeleteAllLibpod<'a>>,
    ) -> Result<super::super::models::LibpodImagesRemoveReport, Error> {
        let mut request_url = url::Url::parse(self.get_config().get_base_path())?;
        let mut request_path = request_url.path().to_owned();
        if request_path.ends_with('/') {
            request_path.pop();
        }
        request_path.push_str("/libpod/images/remove");
        request_url.set_path(&request_path);
        let mut req_builder = self.get_config().req_builder("DELETE")?;
        if let Some(params) = params {
            let mut query_pairs = request_url.query_pairs_mut();
            if let Some(images) = params.images {
                query_pairs.append_pair(
                    "images",
                    &images
                        .iter()
                        .map(|e| e.to_string())
                        .collect::<Vec<_>>()
                        .join(","),
                );
            }
            if let Some(all) = params.all {
                query_pairs.append_pair("all", &all.to_string());
            }
            if let Some(force) = params.force {
                query_pairs.append_pair("force", &force.to_string());
            }
            if let Some(ignore) = params.ignore {
                query_pairs.append_pair("ignore", &ignore.to_string());
            }
            if let Some(lookup_manifest) = params.lookup_manifest {
                query_pairs.append_pair("lookupManifest", &lookup_manifest.to_string());
            }
        }
        let hyper_uri: hyper::Uri = request_url.as_str().parse()?;
        req_builder = req_builder.uri(hyper_uri);
        let request = req_builder.body(String::new())?;
        request::execute_request_json(self.get_config(), request).await
    }
    /// POST /libpod/images/scp/{name}
    /// Copy an image from one host to another
    /// Copy an image from one host to another
    async fn image_scp_libpod<'a>(
        &self,
        name: &str,
        params: Option<super::super::params::ImageScpLibpod<'a>>,
    ) -> Result<super::super::models::ScpReport, Error> {
        let mut request_url = url::Url::parse(self.get_config().get_base_path())?;
        let mut request_path = request_url.path().to_owned();
        if request_path.ends_with('/') {
            request_path.pop();
        }
        request_path.push_str("/libpod/images/scp/{name}");
        request_path = request_path.replace("{name}", name);
        request_url.set_path(&request_path);
        let mut req_builder = self.get_config().req_builder("POST")?;
        if let Some(params) = params {
            let mut query_pairs = request_url.query_pairs_mut();
            if let Some(destination) = params.destination {
                query_pairs.append_pair("destination", destination);
            }
            if let Some(quiet) = params.quiet.or(Some(true)) {
                query_pairs.append_pair("quiet", &quiet.to_string());
            }
        }
        let hyper_uri: hyper::Uri = request_url.as_str().parse()?;
        req_builder = req_builder.uri(hyper_uri);
        let request = req_builder.body(String::new())?;
        request::execute_request_json(self.get_config(), request).await
    }
    /// GET /libpod/images/search
    /// Search images
    /// Search registries for images
    async fn image_search_libpod<'a>(
        &self,
        params: Option<super::super::params::ImageSearchLibpod<'a>>,
    ) -> Result<super::super::models::RegistrySearchResponse, Error> {
        let mut request_url = url::Url::parse(self.get_config().get_base_path())?;
        let mut request_path = request_url.path().to_owned();
        if request_path.ends_with('/') {
            request_path.pop();
        }
        request_path.push_str("/libpod/images/search");
        request_url.set_path(&request_path);
        let mut req_builder = self.get_config().req_builder("GET")?;
        if let Some(params) = params {
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
        let request = req_builder.body(String::new())?;
        request::execute_request_json(self.get_config(), request).await
    }
}

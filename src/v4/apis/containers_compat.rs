use crate::v4::config::HasConfig;
use crate::v4::request;
use crate::v4::Error;
#[async_trait::async_trait]
pub trait ContainersCompat: HasConfig + Send + Sync {
    /// POST /commit
    /// New Image
    /// Create a new image from a container
    async fn image_commit<'a>(
        &self,
        params: Option<super::super::params::ImageCommit<'a>>,
    ) -> Result<(), Error> {
        let mut request_url = url::Url::parse(self.get_config().get_base_path())?;
        let mut request_path = request_url.path().to_owned();
        if request_path.ends_with('/') {
            request_path.pop();
        }
        request_path.push_str("/commit");
        request_url.set_path(&request_path);
        let mut req_builder = self.get_config().req_builder("POST")?;
        if let Some(params) = params {
            let mut query_pairs = request_url.query_pairs_mut();
            if let Some(container) = params.container {
                query_pairs.append_pair("container", container);
            }
            if let Some(repo) = params.repo {
                query_pairs.append_pair("repo", repo);
            }
            if let Some(tag) = params.tag {
                query_pairs.append_pair("tag", tag);
            }
            if let Some(comment) = params.comment {
                query_pairs.append_pair("comment", comment);
            }
            if let Some(author) = params.author {
                query_pairs.append_pair("author", author);
            }
            if let Some(pause) = params.pause {
                query_pairs.append_pair("pause", &pause.to_string());
            }
            if let Some(changes) = params.changes {
                query_pairs.append_pair("changes", changes);
            }
            if let Some(squash) = params.squash {
                query_pairs.append_pair("squash", &squash.to_string());
            }
        }
        let hyper_uri: hyper::Uri = request_url.as_str().parse()?;
        req_builder = req_builder.uri(hyper_uri);
        let request = req_builder.body(String::new())?;
        request::execute_request_unit(self.get_config(), request).await
    }
    /// DELETE /containers/{name}
    /// Remove a container
    async fn container_delete(
        &self,
        name: &str,
        params: Option<super::super::params::ContainerDelete>,
    ) -> Result<(), Error> {
        let mut request_url = url::Url::parse(self.get_config().get_base_path())?;
        let mut request_path = request_url.path().to_owned();
        if request_path.ends_with('/') {
            request_path.pop();
        }
        request_path.push_str("/containers/{name}");
        request_path = request_path.replace("{name}", name);
        request_url.set_path(&request_path);
        let mut req_builder = self.get_config().req_builder("DELETE")?;
        if let Some(params) = params {
            let mut query_pairs = request_url.query_pairs_mut();
            if let Some(force) = params.force {
                query_pairs.append_pair("force", &force.to_string());
            }
            if let Some(v) = params.v {
                query_pairs.append_pair("v", &v.to_string());
            }
            if let Some(link) = params.link {
                query_pairs.append_pair("link", &link.to_string());
            }
        }
        let hyper_uri: hyper::Uri = request_url.as_str().parse()?;
        req_builder = req_builder.uri(hyper_uri);
        let request = req_builder.body(String::new())?;
        request::execute_request_unit(self.get_config(), request).await
    }
    /// GET /containers/{name}/archive
    /// Get files from a container
    /// Get a tar archive of files from a container
    async fn container_archive<'a>(
        &self,
        name: &str,
        params: Option<super::super::params::ContainerArchive<'a>>,
    ) -> Result<String, Error> {
        let mut request_url = url::Url::parse(self.get_config().get_base_path())?;
        let mut request_path = request_url.path().to_owned();
        if request_path.ends_with('/') {
            request_path.pop();
        }
        request_path.push_str("/containers/{name}/archive");
        request_path = request_path.replace("{name}", name);
        request_url.set_path(&request_path);
        let mut req_builder = self.get_config().req_builder("GET")?;
        if let Some(params) = params {
            let mut query_pairs = request_url.query_pairs_mut();
            query_pairs.append_pair("path", params.path);
        }
        let hyper_uri: hyper::Uri = request_url.as_str().parse()?;
        req_builder = req_builder.uri(hyper_uri);
        let request = req_builder.body(String::new())?;
        request::execute_request_json(self.get_config(), request).await
    }
    /// GET /containers/{name}/archive
    /// Put files into a container
    /// Put a tar archive of files into a container
    async fn put_container_archive<'a>(
        &self,
        name: &str,
        params: Option<super::super::params::PutContainerArchive<'a>>,
        request: String,
    ) -> Result<(), Error> {
        let mut request_url = url::Url::parse(self.get_config().get_base_path())?;
        let mut request_path = request_url.path().to_owned();
        if request_path.ends_with('/') {
            request_path.pop();
        }
        request_path.push_str("/containers/{name}/archive");
        request_path = request_path.replace("{name}", name);
        request_url.set_path(&request_path);
        let mut req_builder = self.get_config().req_builder("GET")?;
        if let Some(params) = params {
            let mut query_pairs = request_url.query_pairs_mut();
            query_pairs.append_pair("path", params.path);
            if let Some(no_overwrite_dir_non_dir) = params.no_overwrite_dir_non_dir {
                query_pairs.append_pair("noOverwriteDirNonDir", no_overwrite_dir_non_dir);
            }
            if let Some(copy_uidgid) = params.copy_uidgid {
                query_pairs.append_pair("copyUIDGID", copy_uidgid);
            }
        }
        let hyper_uri: hyper::Uri = request_url.as_str().parse()?;
        req_builder = req_builder.uri(hyper_uri);
        let body = serde_json::to_string(&request)?;
        req_builder = req_builder.header(hyper::header::CONTENT_TYPE, "application/json");
        req_builder = req_builder.header(hyper::header::CONTENT_LENGTH, body.len());
        let request = req_builder.body(body)?;
        request::execute_request_unit(self.get_config(), request).await
    }
    /// POST /containers/{name}/attach
    /// Attach to a container
    /// Attach to a container to read its output or send it input. You can attach
    /// to the same container multiple times and you can reattach to containers
    /// that have been detached.
    ///
    /// It uses the same stream format as docker, see the libpod attach endpoint for a description of the format.
    async fn container_attach<'a>(
        &self,
        name: &str,
        params: Option<super::super::params::ContainerAttach<'a>>,
    ) -> Result<(), Error> {
        let mut request_url = url::Url::parse(self.get_config().get_base_path())?;
        let mut request_path = request_url.path().to_owned();
        if request_path.ends_with('/') {
            request_path.pop();
        }
        request_path.push_str("/containers/{name}/attach");
        request_path = request_path.replace("{name}", name);
        request_url.set_path(&request_path);
        let mut req_builder = self.get_config().req_builder("POST")?;
        if let Some(params) = params {
            let mut query_pairs = request_url.query_pairs_mut();
            if let Some(detach_keys) = params.detach_keys {
                query_pairs.append_pair("detachKeys", detach_keys);
            }
            if let Some(logs) = params.logs {
                query_pairs.append_pair("logs", &logs.to_string());
            }
            if let Some(stream) = params.stream {
                query_pairs.append_pair("stream", &stream.to_string());
            }
            if let Some(stdout) = params.stdout {
                query_pairs.append_pair("stdout", &stdout.to_string());
            }
            if let Some(stderr) = params.stderr {
                query_pairs.append_pair("stderr", &stderr.to_string());
            }
            if let Some(stdin) = params.stdin {
                query_pairs.append_pair("stdin", &stdin.to_string());
            }
        }
        let hyper_uri: hyper::Uri = request_url.as_str().parse()?;
        req_builder = req_builder.uri(hyper_uri);
        let request = req_builder.body(String::new())?;
        request::execute_request_unit(self.get_config(), request).await
    }
    /// GET /containers/{name}/export
    /// Export a container
    /// Export the contents of a container as a tarball.
    async fn container_export(&self, name: &str) -> Result<(), Error> {
        let mut request_url = url::Url::parse(self.get_config().get_base_path())?;
        let mut request_path = request_url.path().to_owned();
        if request_path.ends_with('/') {
            request_path.pop();
        }
        request_path.push_str("/containers/{name}/export");
        request_path = request_path.replace("{name}", name);
        request_url.set_path(&request_path);
        let mut req_builder = self.get_config().req_builder("GET")?;
        let hyper_uri: hyper::Uri = request_url.as_str().parse()?;
        req_builder = req_builder.uri(hyper_uri);
        let request = req_builder.body(String::new())?;
        request::execute_request_unit(self.get_config(), request).await
    }
    /// GET /containers/{name}/json
    /// Inspect container
    /// Return low-level information about a container.
    async fn container_inspect(
        &self,
        name: &str,
        params: Option<super::super::params::ContainerInspect>,
    ) -> Result<(), Error> {
        let mut request_url = url::Url::parse(self.get_config().get_base_path())?;
        let mut request_path = request_url.path().to_owned();
        if request_path.ends_with('/') {
            request_path.pop();
        }
        request_path.push_str("/containers/{name}/json");
        request_path = request_path.replace("{name}", name);
        request_url.set_path(&request_path);
        let mut req_builder = self.get_config().req_builder("GET")?;
        if let Some(params) = params {
            let mut query_pairs = request_url.query_pairs_mut();
            if let Some(size) = params.size {
                query_pairs.append_pair("size", &size.to_string());
            }
        }
        let hyper_uri: hyper::Uri = request_url.as_str().parse()?;
        req_builder = req_builder.uri(hyper_uri);
        let request = req_builder.body(String::new())?;
        request::execute_request_unit(self.get_config(), request).await
    }
    /// POST /containers/{name}/kill
    /// Kill container
    /// Signal to send to the container as an integer or string (e.g. SIGINT)
    async fn container_kill<'a>(
        &self,
        name: &str,
        params: Option<super::super::params::ContainerKill<'a>>,
    ) -> Result<(), Error> {
        let mut request_url = url::Url::parse(self.get_config().get_base_path())?;
        let mut request_path = request_url.path().to_owned();
        if request_path.ends_with('/') {
            request_path.pop();
        }
        request_path.push_str("/containers/{name}/kill");
        request_path = request_path.replace("{name}", name);
        request_url.set_path(&request_path);
        let mut req_builder = self.get_config().req_builder("POST")?;
        if let Some(params) = params {
            let mut query_pairs = request_url.query_pairs_mut();
            if let Some(all) = params.all {
                query_pairs.append_pair("all", &all.to_string());
            }
            if let Some(signal) = params.signal {
                query_pairs.append_pair("signal", signal);
            }
        }
        let hyper_uri: hyper::Uri = request_url.as_str().parse()?;
        req_builder = req_builder.uri(hyper_uri);
        let request = req_builder.body(String::new())?;
        request::execute_request_unit(self.get_config(), request).await
    }
    /// GET /containers/{name}/logs
    /// Get container logs
    /// Get stdout and stderr logs from a container.
    async fn container_logs<'a>(
        &self,
        name: &str,
        params: Option<super::super::params::ContainerLogs<'a>>,
    ) -> Result<(), Error> {
        let mut request_url = url::Url::parse(self.get_config().get_base_path())?;
        let mut request_path = request_url.path().to_owned();
        if request_path.ends_with('/') {
            request_path.pop();
        }
        request_path.push_str("/containers/{name}/logs");
        request_path = request_path.replace("{name}", name);
        request_url.set_path(&request_path);
        let mut req_builder = self.get_config().req_builder("GET")?;
        if let Some(params) = params {
            let mut query_pairs = request_url.query_pairs_mut();
            if let Some(follow) = params.follow {
                query_pairs.append_pair("follow", &follow.to_string());
            }
            if let Some(stdout) = params.stdout {
                query_pairs.append_pair("stdout", &stdout.to_string());
            }
            if let Some(stderr) = params.stderr {
                query_pairs.append_pair("stderr", &stderr.to_string());
            }
            if let Some(since) = params.since {
                query_pairs.append_pair("since", since);
            }
            if let Some(until) = params.until {
                query_pairs.append_pair("until", until);
            }
            if let Some(timestamps) = params.timestamps {
                query_pairs.append_pair("timestamps", &timestamps.to_string());
            }
            if let Some(tail) = params.tail {
                query_pairs.append_pair("tail", tail);
            }
        }
        let hyper_uri: hyper::Uri = request_url.as_str().parse()?;
        req_builder = req_builder.uri(hyper_uri);
        let request = req_builder.body(String::new())?;
        request::execute_request_unit(self.get_config(), request).await
    }
    /// POST /containers/{name}/pause
    /// Pause container
    /// Use the cgroups freezer to suspend all processes in a container.
    async fn container_pause(&self, name: &str) -> Result<(), Error> {
        let mut request_url = url::Url::parse(self.get_config().get_base_path())?;
        let mut request_path = request_url.path().to_owned();
        if request_path.ends_with('/') {
            request_path.pop();
        }
        request_path.push_str("/containers/{name}/pause");
        request_path = request_path.replace("{name}", name);
        request_url.set_path(&request_path);
        let mut req_builder = self.get_config().req_builder("POST")?;
        let hyper_uri: hyper::Uri = request_url.as_str().parse()?;
        req_builder = req_builder.uri(hyper_uri);
        let request = req_builder.body(String::new())?;
        request::execute_request_unit(self.get_config(), request).await
    }
    /// POST /containers/{name}/rename
    /// Rename an existing container
    /// Change the name of an existing container.
    async fn container_rename<'a>(
        &self,
        name: &str,
        params: Option<super::super::params::ContainerRename<'a>>,
    ) -> Result<(), Error> {
        let mut request_url = url::Url::parse(self.get_config().get_base_path())?;
        let mut request_path = request_url.path().to_owned();
        if request_path.ends_with('/') {
            request_path.pop();
        }
        request_path.push_str("/containers/{name}/rename");
        request_path = request_path.replace("{name}", name);
        request_url.set_path(&request_path);
        let mut req_builder = self.get_config().req_builder("POST")?;
        if let Some(params) = params {
            let mut query_pairs = request_url.query_pairs_mut();
            query_pairs.append_pair("name", params.name);
        }
        let hyper_uri: hyper::Uri = request_url.as_str().parse()?;
        req_builder = req_builder.uri(hyper_uri);
        let request = req_builder.body(String::new())?;
        request::execute_request_unit(self.get_config(), request).await
    }
    /// POST /containers/{name}/resize
    /// Resize a container's TTY
    /// Resize the terminal attached to a container (for use with Attach).
    async fn container_resize(
        &self,
        name: &str,
        params: Option<super::super::params::ContainerResize>,
    ) -> Result<(), Error> {
        let mut request_url = url::Url::parse(self.get_config().get_base_path())?;
        let mut request_path = request_url.path().to_owned();
        if request_path.ends_with('/') {
            request_path.pop();
        }
        request_path.push_str("/containers/{name}/resize");
        request_path = request_path.replace("{name}", name);
        request_url.set_path(&request_path);
        let mut req_builder = self.get_config().req_builder("POST")?;
        if let Some(params) = params {
            let mut query_pairs = request_url.query_pairs_mut();
            if let Some(h) = params.h {
                query_pairs.append_pair("h", &h.to_string());
            }
            if let Some(w) = params.w {
                query_pairs.append_pair("w", &w.to_string());
            }
            if let Some(running) = params.running {
                query_pairs.append_pair("running", &running.to_string());
            }
        }
        let hyper_uri: hyper::Uri = request_url.as_str().parse()?;
        req_builder = req_builder.uri(hyper_uri);
        let request = req_builder.body(String::new())?;
        request::execute_request_unit(self.get_config(), request).await
    }
    /// POST /containers/{name}/restart
    /// Restart container
    async fn container_restart(
        &self,
        name: &str,
        params: Option<super::super::params::ContainerRestart>,
    ) -> Result<(), Error> {
        let mut request_url = url::Url::parse(self.get_config().get_base_path())?;
        let mut request_path = request_url.path().to_owned();
        if request_path.ends_with('/') {
            request_path.pop();
        }
        request_path.push_str("/containers/{name}/restart");
        request_path = request_path.replace("{name}", name);
        request_url.set_path(&request_path);
        let mut req_builder = self.get_config().req_builder("POST")?;
        if let Some(params) = params {
            let mut query_pairs = request_url.query_pairs_mut();
            if let Some(t) = params.t {
                query_pairs.append_pair("t", &t.to_string());
            }
        }
        let hyper_uri: hyper::Uri = request_url.as_str().parse()?;
        req_builder = req_builder.uri(hyper_uri);
        let request = req_builder.body(String::new())?;
        request::execute_request_unit(self.get_config(), request).await
    }
    /// POST /containers/{name}/start
    /// Start a container
    async fn container_start<'a>(
        &self,
        name: &str,
        params: Option<super::super::params::ContainerStart<'a>>,
    ) -> Result<(), Error> {
        let mut request_url = url::Url::parse(self.get_config().get_base_path())?;
        let mut request_path = request_url.path().to_owned();
        if request_path.ends_with('/') {
            request_path.pop();
        }
        request_path.push_str("/containers/{name}/start");
        request_path = request_path.replace("{name}", name);
        request_url.set_path(&request_path);
        let mut req_builder = self.get_config().req_builder("POST")?;
        if let Some(params) = params {
            let mut query_pairs = request_url.query_pairs_mut();
            if let Some(detach_keys) = params.detach_keys {
                query_pairs.append_pair("detachKeys", detach_keys);
            }
        }
        let hyper_uri: hyper::Uri = request_url.as_str().parse()?;
        req_builder = req_builder.uri(hyper_uri);
        let request = req_builder.body(String::new())?;
        request::execute_request_unit(self.get_config(), request).await
    }
    /// GET /containers/{name}/stats
    /// Get stats for a container
    /// This returns a live stream of a container’s resource usage statistics.
    async fn container_stats(
        &self,
        name: &str,
        params: Option<super::super::params::ContainerStats>,
    ) -> Result<serde_json::Value, Error> {
        let mut request_url = url::Url::parse(self.get_config().get_base_path())?;
        let mut request_path = request_url.path().to_owned();
        if request_path.ends_with('/') {
            request_path.pop();
        }
        request_path.push_str("/containers/{name}/stats");
        request_path = request_path.replace("{name}", name);
        request_url.set_path(&request_path);
        let mut req_builder = self.get_config().req_builder("GET")?;
        if let Some(params) = params {
            let mut query_pairs = request_url.query_pairs_mut();
            if let Some(stream) = params.stream {
                query_pairs.append_pair("stream", &stream.to_string());
            }
            if let Some(one_shot) = params.one_shot {
                query_pairs.append_pair("one-shot", &one_shot.to_string());
            }
        }
        let hyper_uri: hyper::Uri = request_url.as_str().parse()?;
        req_builder = req_builder.uri(hyper_uri);
        let request = req_builder.body(String::new())?;
        request::execute_request_json(self.get_config(), request).await
    }
    /// POST /containers/{name}/stop
    /// Stop a container
    /// Stop a container
    async fn container_stop(
        &self,
        name: &str,
        params: Option<super::super::params::ContainerStop>,
    ) -> Result<(), Error> {
        let mut request_url = url::Url::parse(self.get_config().get_base_path())?;
        let mut request_path = request_url.path().to_owned();
        if request_path.ends_with('/') {
            request_path.pop();
        }
        request_path.push_str("/containers/{name}/stop");
        request_path = request_path.replace("{name}", name);
        request_url.set_path(&request_path);
        let mut req_builder = self.get_config().req_builder("POST")?;
        if let Some(params) = params {
            let mut query_pairs = request_url.query_pairs_mut();
            if let Some(t) = params.t {
                query_pairs.append_pair("t", &t.to_string());
            }
        }
        let hyper_uri: hyper::Uri = request_url.as_str().parse()?;
        req_builder = req_builder.uri(hyper_uri);
        let request = req_builder.body(String::new())?;
        request::execute_request_unit(self.get_config(), request).await
    }
    /// GET /containers/{name}/top
    /// List processes running inside a container
    async fn container_top<'a>(
        &self,
        name: &str,
        params: Option<super::super::params::ContainerTop<'a>>,
    ) -> Result<(), Error> {
        let mut request_url = url::Url::parse(self.get_config().get_base_path())?;
        let mut request_path = request_url.path().to_owned();
        if request_path.ends_with('/') {
            request_path.pop();
        }
        request_path.push_str("/containers/{name}/top");
        request_path = request_path.replace("{name}", name);
        request_url.set_path(&request_path);
        let mut req_builder = self.get_config().req_builder("GET")?;
        if let Some(params) = params {
            let mut query_pairs = request_url.query_pairs_mut();
            if let Some(ps_args) = params.ps_args {
                query_pairs.append_pair("ps_args", ps_args);
            }
        }
        let hyper_uri: hyper::Uri = request_url.as_str().parse()?;
        req_builder = req_builder.uri(hyper_uri);
        let request = req_builder.body(String::new())?;
        request::execute_request_unit(self.get_config(), request).await
    }
    /// POST /containers/{name}/unpause
    /// Unpause container
    /// Resume a paused container
    async fn container_unpause(&self, name: &str) -> Result<(), Error> {
        let mut request_url = url::Url::parse(self.get_config().get_base_path())?;
        let mut request_path = request_url.path().to_owned();
        if request_path.ends_with('/') {
            request_path.pop();
        }
        request_path.push_str("/containers/{name}/unpause");
        request_path = request_path.replace("{name}", name);
        request_url.set_path(&request_path);
        let mut req_builder = self.get_config().req_builder("POST")?;
        let hyper_uri: hyper::Uri = request_url.as_str().parse()?;
        req_builder = req_builder.uri(hyper_uri);
        let request = req_builder.body(String::new())?;
        request::execute_request_unit(self.get_config(), request).await
    }
    /// POST /containers/{name}/wait
    /// Wait on a container
    /// Block until a container stops or given condition is met.
    async fn container_wait<'a>(
        &self,
        name: &str,
        params: Option<super::super::params::ContainerWait<'a>>,
    ) -> Result<(), Error> {
        let mut request_url = url::Url::parse(self.get_config().get_base_path())?;
        let mut request_path = request_url.path().to_owned();
        if request_path.ends_with('/') {
            request_path.pop();
        }
        request_path.push_str("/containers/{name}/wait");
        request_path = request_path.replace("{name}", name);
        request_url.set_path(&request_path);
        let mut req_builder = self.get_config().req_builder("POST")?;
        if let Some(params) = params {
            let mut query_pairs = request_url.query_pairs_mut();
            if let Some(condition) = params.condition {
                query_pairs.append_pair("condition", condition);
            }
            if let Some(interval) = params.interval {
                query_pairs.append_pair("interval", interval);
            }
        }
        let hyper_uri: hyper::Uri = request_url.as_str().parse()?;
        req_builder = req_builder.uri(hyper_uri);
        let request = req_builder.body(String::new())?;
        request::execute_request_unit(self.get_config(), request).await
    }
    /// POST /containers/create
    /// Create a container
    async fn container_create<'a>(
        &self,
        params: Option<super::super::params::ContainerCreate<'a>>,
        body: (),
    ) -> Result<(), Error> {
        let mut request_url = url::Url::parse(self.get_config().get_base_path())?;
        let mut request_path = request_url.path().to_owned();
        if request_path.ends_with('/') {
            request_path.pop();
        }
        request_path.push_str("/containers/create");
        request_url.set_path(&request_path);
        let mut req_builder = self.get_config().req_builder("POST")?;
        if let Some(params) = params {
            let mut query_pairs = request_url.query_pairs_mut();
            if let Some(name) = params.name {
                query_pairs.append_pair("name", name);
            }
        }
        let hyper_uri: hyper::Uri = request_url.as_str().parse()?;
        req_builder = req_builder.uri(hyper_uri);
        let body = serde_json::to_string(&body)?;
        req_builder = req_builder.header(hyper::header::CONTENT_TYPE, "application/json");
        req_builder = req_builder.header(hyper::header::CONTENT_LENGTH, body.len());
        let request = req_builder.body(body)?;
        request::execute_request_unit(self.get_config(), request).await
    }
    /// GET /containers/json
    /// List containers
    /// Returns a list of containers
    async fn container_list<'a>(
        &self,
        params: Option<super::super::params::ContainerList<'a>>,
    ) -> Result<Vec<()>, Error> {
        let mut request_url = url::Url::parse(self.get_config().get_base_path())?;
        let mut request_path = request_url.path().to_owned();
        if request_path.ends_with('/') {
            request_path.pop();
        }
        request_path.push_str("/containers/json");
        request_url.set_path(&request_path);
        let mut req_builder = self.get_config().req_builder("GET")?;
        if let Some(params) = params {
            let mut query_pairs = request_url.query_pairs_mut();
            if let Some(all) = params.all {
                query_pairs.append_pair("all", &all.to_string());
            }
            if let Some(external) = params.external {
                query_pairs.append_pair("external", &external.to_string());
            }
            if let Some(limit) = params.limit {
                query_pairs.append_pair("limit", &limit.to_string());
            }
            if let Some(size) = params.size {
                query_pairs.append_pair("size", &size.to_string());
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
    /// POST /containers/prune
    /// Delete stopped containers
    /// Remove containers not in use
    async fn container_prune<'a>(
        &self,
        params: Option<super::super::params::ContainerPrune<'a>>,
    ) -> Result<Vec<()>, Error> {
        let mut request_url = url::Url::parse(self.get_config().get_base_path())?;
        let mut request_path = request_url.path().to_owned();
        if request_path.ends_with('/') {
            request_path.pop();
        }
        request_path.push_str("/containers/prune");
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
    /// GET /libpod/containers/{name}/archive
    /// Copy files from a container
    /// Copy a tar archive of files from a container
    async fn container_archive_libpod<'a>(
        &self,
        name: &str,
        params: Option<super::super::params::ContainerArchiveLibpod<'a>>,
    ) -> Result<String, Error> {
        let mut request_url = url::Url::parse(self.get_config().get_base_path())?;
        let mut request_path = request_url.path().to_owned();
        if request_path.ends_with('/') {
            request_path.pop();
        }
        request_path.push_str("/libpod/containers/{name}/archive");
        request_path = request_path.replace("{name}", name);
        request_url.set_path(&request_path);
        let mut req_builder = self.get_config().req_builder("GET")?;
        if let Some(params) = params {
            let mut query_pairs = request_url.query_pairs_mut();
            query_pairs.append_pair("path", params.path);
            if let Some(rename) = params.rename {
                query_pairs.append_pair("rename", rename);
            }
        }
        let hyper_uri: hyper::Uri = request_url.as_str().parse()?;
        req_builder = req_builder.uri(hyper_uri);
        let request = req_builder.body(String::new())?;
        request::execute_request_json(self.get_config(), request).await
    }
}

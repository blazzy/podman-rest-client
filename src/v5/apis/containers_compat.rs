use crate::api_common::config::HasConfig;
use crate::api_common::request;
use crate::api_common::Error;
use std::future::Future;
use std::pin::Pin;
pub trait ContainersCompat: HasConfig + Send + Sync {
    /// POST /commit
    /// New Image
    /// Create a new image from a container
    fn image_commit<'a>(
        &'a self,
        params: Option<crate::v5::params::ImageCommit<'a>>,
    ) -> Pin<Box<dyn Future<Output = Result<(), Error>> + Send + 'a>> {
        Box::pin(request::execute_request_unit(
            self.get_config(),
            (|| {
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
                Ok(req_builder.body(String::new())?)
            })(),
        ))
    }
    /// DELETE /containers/{name}
    /// Remove a container
    fn container_delete<'a>(
        &'a self,
        name: &'a str,
        params: Option<crate::v5::params::ContainerDelete>,
    ) -> Pin<Box<dyn Future<Output = Result<(), Error>> + Send + 'a>> {
        Box::pin(request::execute_request_unit(
            self.get_config(),
            (|| {
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
                Ok(req_builder.body(String::new())?)
            })(),
        ))
    }
    /// GET /containers/{name}/archive
    /// Get files from a container
    /// Get a tar archive of files from a container
    fn container_archive<'a>(
        &'a self,
        name: &'a str,
        params: Option<crate::v5::params::ContainerArchive<'a>>,
    ) -> Pin<Box<dyn Future<Output = Result<String, Error>> + Send + 'a>> {
        Box::pin(request::execute_request_json(
            self.get_config(),
            (|| {
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
                Ok(req_builder.body(String::new())?)
            })(),
        ))
    }
    /// GET /containers/{name}/archive
    /// Put files into a container
    /// Put a tar archive of files into a container
    fn put_container_archive<'a>(
        &'a self,
        name: &'a str,
        params: Option<crate::v5::params::PutContainerArchive<'a>>,
        request: String,
    ) -> Pin<Box<dyn Future<Output = Result<(), Error>> + Send + 'a>> {
        Box::pin(request::execute_request_unit(
            self.get_config(),
            (|| {
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
                Ok(req_builder.body(body)?)
            })(),
        ))
    }
    /// POST /containers/{name}/attach
    /// Attach to a container
    /// Attach to a container to read its output or send it input. You can attach
    /// to the same container multiple times and you can reattach to containers
    /// that have been detached.
    ///
    /// It uses the same stream format as docker, see the libpod attach endpoint for a description of the format.
    fn container_attach<'a>(
        &'a self,
        name: &'a str,
        params: Option<crate::v5::params::ContainerAttach<'a>>,
    ) -> Pin<Box<dyn Future<Output = Result<(), Error>> + Send + 'a>> {
        Box::pin(request::execute_request_unit(
            self.get_config(),
            (|| {
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
                Ok(req_builder.body(String::new())?)
            })(),
        ))
    }
    /// GET /containers/{name}/export
    /// Export a container
    /// Export the contents of a container as a tarball.
    fn container_export<'a>(
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
                request_path.push_str("/containers/{name}/export");
                request_path = request_path.replace("{name}", name);
                request_url.set_path(&request_path);
                let mut req_builder = self.get_config().req_builder("GET")?;
                let hyper_uri: hyper::Uri = request_url.as_str().parse()?;
                req_builder = req_builder.uri(hyper_uri);
                Ok(req_builder.body(String::new())?)
            })(),
        ))
    }
    /// GET /containers/{name}/json
    /// Inspect container
    /// Return low-level information about a container.
    fn container_inspect<'a>(
        &'a self,
        name: &'a str,
        params: Option<crate::v5::params::ContainerInspect>,
    ) -> Pin<Box<dyn Future<Output = Result<crate::v5::models::ContainerJson, Error>> + Send + 'a>>
    {
        Box::pin(request::execute_request_json(
            self.get_config(),
            (|| {
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
                Ok(req_builder.body(String::new())?)
            })(),
        ))
    }
    /// POST /containers/{name}/kill
    /// Kill container
    /// Signal to send to the container as an integer or string (e.g. SIGINT)
    fn container_kill<'a>(
        &'a self,
        name: &'a str,
        params: Option<crate::v5::params::ContainerKill<'a>>,
    ) -> Pin<Box<dyn Future<Output = Result<(), Error>> + Send + 'a>> {
        Box::pin(request::execute_request_unit(
            self.get_config(),
            (|| {
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
                Ok(req_builder.body(String::new())?)
            })(),
        ))
    }
    /// GET /containers/{name}/logs
    /// Get container logs
    /// Get stdout and stderr logs from a container.
    fn container_logs<'a>(
        &'a self,
        name: &'a str,
        params: Option<crate::v5::params::ContainerLogs<'a>>,
    ) -> Pin<Box<dyn Future<Output = Result<(), Error>> + Send + 'a>> {
        Box::pin(request::execute_request_unit(
            self.get_config(),
            (|| {
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
                Ok(req_builder.body(String::new())?)
            })(),
        ))
    }
    /// POST /containers/{name}/pause
    /// Pause container
    /// Use the cgroups freezer to suspend all processes in a container.
    fn container_pause<'a>(
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
                request_path.push_str("/containers/{name}/pause");
                request_path = request_path.replace("{name}", name);
                request_url.set_path(&request_path);
                let mut req_builder = self.get_config().req_builder("POST")?;
                let hyper_uri: hyper::Uri = request_url.as_str().parse()?;
                req_builder = req_builder.uri(hyper_uri);
                Ok(req_builder.body(String::new())?)
            })(),
        ))
    }
    /// POST /containers/{name}/rename
    /// Rename an existing container
    /// Change the name of an existing container.
    fn container_rename<'a>(
        &'a self,
        name: &'a str,
        params: Option<crate::v5::params::ContainerRename<'a>>,
    ) -> Pin<Box<dyn Future<Output = Result<(), Error>> + Send + 'a>> {
        Box::pin(request::execute_request_unit(
            self.get_config(),
            (|| {
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
                Ok(req_builder.body(String::new())?)
            })(),
        ))
    }
    /// POST /containers/{name}/resize
    /// Resize a container's TTY
    /// Resize the terminal attached to a container (for use with Attach).
    fn container_resize<'a>(
        &'a self,
        name: &'a str,
        params: Option<crate::v5::params::ContainerResize>,
    ) -> Pin<Box<dyn Future<Output = Result<serde_json::Value, Error>> + Send + 'a>> {
        Box::pin(request::execute_request_json(
            self.get_config(),
            (|| {
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
                Ok(req_builder.body(String::new())?)
            })(),
        ))
    }
    /// POST /containers/{name}/restart
    /// Restart container
    fn container_restart<'a>(
        &'a self,
        name: &'a str,
        params: Option<crate::v5::params::ContainerRestart>,
    ) -> Pin<Box<dyn Future<Output = Result<(), Error>> + Send + 'a>> {
        Box::pin(request::execute_request_unit(
            self.get_config(),
            (|| {
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
                Ok(req_builder.body(String::new())?)
            })(),
        ))
    }
    /// POST /containers/{name}/start
    /// Start a container
    fn container_start<'a>(
        &'a self,
        name: &'a str,
        params: Option<crate::v5::params::ContainerStart<'a>>,
    ) -> Pin<Box<dyn Future<Output = Result<(), Error>> + Send + 'a>> {
        Box::pin(request::execute_request_unit(
            self.get_config(),
            (|| {
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
                Ok(req_builder.body(String::new())?)
            })(),
        ))
    }
    /// GET /containers/{name}/stats
    /// Get stats for a container
    /// This returns a live stream of a container’s resource usage statistics.
    fn container_stats<'a>(
        &'a self,
        name: &'a str,
        params: Option<crate::v5::params::ContainerStats>,
    ) -> Pin<Box<dyn Future<Output = Result<serde_json::Value, Error>> + Send + 'a>> {
        Box::pin(request::execute_request_json(
            self.get_config(),
            (|| {
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
                Ok(req_builder.body(String::new())?)
            })(),
        ))
    }
    /// POST /containers/{name}/stop
    /// Stop a container
    /// Stop a container
    fn container_stop<'a>(
        &'a self,
        name: &'a str,
        params: Option<crate::v5::params::ContainerStop>,
    ) -> Pin<Box<dyn Future<Output = Result<(), Error>> + Send + 'a>> {
        Box::pin(request::execute_request_unit(
            self.get_config(),
            (|| {
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
                Ok(req_builder.body(String::new())?)
            })(),
        ))
    }
    /// GET /containers/{name}/top
    /// List processes running inside a container
    fn container_top<'a>(
        &'a self,
        name: &'a str,
        params: Option<crate::v5::params::ContainerTop<'a>>,
    ) -> Pin<
        Box<dyn Future<Output = Result<crate::v5::models::ContainerTopOkBody, Error>> + Send + 'a>,
    > {
        Box::pin(request::execute_request_json(
            self.get_config(),
            (|| {
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
                Ok(req_builder.body(String::new())?)
            })(),
        ))
    }
    /// POST /containers/{name}/unpause
    /// Unpause container
    /// Resume a paused container
    fn container_unpause<'a>(
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
                request_path.push_str("/containers/{name}/unpause");
                request_path = request_path.replace("{name}", name);
                request_url.set_path(&request_path);
                let mut req_builder = self.get_config().req_builder("POST")?;
                let hyper_uri: hyper::Uri = request_url.as_str().parse()?;
                req_builder = req_builder.uri(hyper_uri);
                Ok(req_builder.body(String::new())?)
            })(),
        ))
    }
    /// POST /containers/{name}/update
    /// Update configuration of an existing container
    /// Change configuration settings for an existing container without requiring recreation.
    fn container_update<'a>(
        &'a self,
        name: &'a str,
        resources: crate::v5::models::UpdateConfig,
    ) -> Pin<Box<dyn Future<Output = Result<(), Error>> + Send + 'a>> {
        Box::pin(request::execute_request_unit(
            self.get_config(),
            (|| {
                let mut request_url = url::Url::parse(self.get_config().get_base_path())?;
                let mut request_path = request_url.path().to_owned();
                if request_path.ends_with('/') {
                    request_path.pop();
                }
                request_path.push_str("/containers/{name}/update");
                request_path = request_path.replace("{name}", name);
                request_url.set_path(&request_path);
                let mut req_builder = self.get_config().req_builder("POST")?;
                let hyper_uri: hyper::Uri = request_url.as_str().parse()?;
                req_builder = req_builder.uri(hyper_uri);
                let body = serde_json::to_string(&resources)?;
                req_builder = req_builder.header(hyper::header::CONTENT_TYPE, "application/json");
                req_builder = req_builder.header(hyper::header::CONTENT_LENGTH, body.len());
                Ok(req_builder.body(body)?)
            })(),
        ))
    }
    /// POST /containers/{name}/wait
    /// Wait on a container
    /// Block until a container stops or given condition is met.
    fn container_wait<'a>(
        &'a self,
        name: &'a str,
        params: Option<crate::v5::params::ContainerWait<'a>>,
    ) -> Pin<
        Box<
            dyn Future<Output = Result<crate::v5::models::ContainerWaitResponse, Error>>
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
                Ok(req_builder.body(String::new())?)
            })(),
        ))
    }
    /// POST /containers/create
    /// Create a container
    fn container_create<'a>(
        &'a self,
        params: Option<crate::v5::params::ContainerCreate<'a>>,
        body: crate::v5::models::CreateContainerConfig,
    ) -> Pin<
        Box<
            dyn Future<Output = Result<crate::v5::models::ContainerCreateResponse, Error>>
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
                Ok(req_builder.body(body)?)
            })(),
        ))
    }
    /// GET /containers/json
    /// List containers
    /// Returns a list of containers
    fn container_list<'a>(
        &'a self,
        params: Option<crate::v5::params::ContainerList<'a>>,
    ) -> Pin<Box<dyn Future<Output = Result<Vec<crate::v5::models::Container>, Error>> + Send + 'a>>
    {
        Box::pin(request::execute_request_json(
            self.get_config(),
            (|| {
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
                Ok(req_builder.body(String::new())?)
            })(),
        ))
    }
    /// POST /containers/prune
    /// Delete stopped containers
    /// Remove containers not in use
    fn container_prune<'a>(
        &'a self,
        params: Option<crate::v5::params::ContainerPrune<'a>>,
    ) -> Pin<
        Box<
            dyn Future<Output = Result<Vec<crate::v5::models::ContainersPruneReport>, Error>>
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
                Ok(req_builder.body(String::new())?)
            })(),
        ))
    }
    /// GET /libpod/containers/{name}/archive
    /// Copy files from a container
    /// Copy a tar archive of files from a container
    fn container_archive_libpod<'a>(
        &'a self,
        name: &'a str,
        params: Option<crate::v5::params::ContainerArchiveLibpod<'a>>,
    ) -> Pin<Box<dyn Future<Output = Result<String, Error>> + Send + 'a>> {
        Box::pin(request::execute_request_json(
            self.get_config(),
            (|| {
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
                Ok(req_builder.body(String::new())?)
            })(),
        ))
    }
}

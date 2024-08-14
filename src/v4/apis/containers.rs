use crate::api_common::config::HasConfig;
use crate::api_common::request;
use crate::api_common::Error;
#[async_trait::async_trait]
pub trait Containers: HasConfig + Send + Sync {
    /// POST /libpod/commit
    /// Commit
    /// Create a new image from a container
    async fn image_commit_libpod<'a>(
        &self,
        params: Option<super::super::params::ImageCommitLibpod<'a>>,
    ) -> Result<(), Error> {
        let mut request_url = url::Url::parse(self.get_config().get_base_path())?;
        let mut request_path = request_url.path().to_owned();
        if request_path.ends_with('/') {
            request_path.pop();
        }
        request_path.push_str("/libpod/commit");
        request_url.set_path(&request_path);
        let mut req_builder = self.get_config().req_builder("POST")?;
        if let Some(params) = params {
            let mut query_pairs = request_url.query_pairs_mut();
            query_pairs.append_pair("container", params.container);
            if let Some(author) = params.author {
                query_pairs.append_pair("author", author);
            }
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
            if let Some(comment) = params.comment {
                query_pairs.append_pair("comment", comment);
            }
            if let Some(format) = params.format {
                query_pairs.append_pair("format", format);
            }
            if let Some(pause) = params.pause {
                query_pairs.append_pair("pause", &pause.to_string());
            }
            if let Some(squash) = params.squash {
                query_pairs.append_pair("squash", &squash.to_string());
            }
            if let Some(repo) = params.repo {
                query_pairs.append_pair("repo", repo);
            }
            if let Some(stream) = params.stream {
                query_pairs.append_pair("stream", &stream.to_string());
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
    /// DELETE /libpod/containers/{name}
    /// Delete container
    /// Delete container
    async fn container_delete_libpod(
        &self,
        name: &str,
        params: Option<super::super::params::ContainerDeleteLibpod>,
    ) -> Result<Vec<()>, Error> {
        let mut request_url = url::Url::parse(self.get_config().get_base_path())?;
        let mut request_path = request_url.path().to_owned();
        if request_path.ends_with('/') {
            request_path.pop();
        }
        request_path.push_str("/libpod/containers/{name}");
        request_path = request_path.replace("{name}", name);
        request_url.set_path(&request_path);
        let mut req_builder = self.get_config().req_builder("DELETE")?;
        if let Some(params) = params {
            let mut query_pairs = request_url.query_pairs_mut();
            if let Some(depend) = params.depend {
                query_pairs.append_pair("depend", &depend.to_string());
            }
            if let Some(force) = params.force {
                query_pairs.append_pair("force", &force.to_string());
            }
            if let Some(ignore) = params.ignore {
                query_pairs.append_pair("ignore", &ignore.to_string());
            }
            if let Some(timeout) = params.timeout {
                query_pairs.append_pair("timeout", &timeout.to_string());
            }
            if let Some(v) = params.v {
                query_pairs.append_pair("v", &v.to_string());
            }
        }
        let hyper_uri: hyper::Uri = request_url.as_str().parse()?;
        req_builder = req_builder.uri(hyper_uri);
        let request = req_builder.body(String::new())?;
        request::execute_request_json(self.get_config(), request).await
    }
    /// GET /libpod/containers/{name}/archive
    /// Copy files into a container
    /// Copy a tar archive of files into a container
    async fn put_container_archive_libpod<'a>(
        &self,
        name: &str,
        params: Option<super::super::params::PutContainerArchiveLibpod<'a>>,
        request: String,
    ) -> Result<(), Error> {
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
            if let Some(pause) = params.pause {
                query_pairs.append_pair("pause", &pause.to_string());
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
    /// POST /libpod/containers/{name}/attach
    /// Attach to a container
    /// Attach to a container to read its output or send it input. You can attach
    /// to the same container multiple times and you can reattach to containers
    /// that have been detached.
    ///
    /// ### Hijacking
    ///
    /// This endpoint hijacks the HTTP connection to transport `stdin`, `stdout`,
    /// and `stderr` on the same socket.
    ///
    /// This is the response from the service for an attach request:
    ///
    /// ```text
    /// HTTP/1.1 200 OK
    /// Content-Type: application/vnd.docker.raw-stream
    ///
    /// [STREAM]
    /// ```
    ///
    /// After the headers and two new lines, the TCP connection can now be used
    /// for raw, bidirectional communication between the client and server.
    ///
    /// To inform potential proxies about connection hijacking, the client
    /// can also optionally send connection upgrade headers.
    ///
    /// For example, the client sends this request to upgrade the connection:
    ///
    /// ```text
    /// POST /v4.6.0/libpod/containers/16253994b7c4/attach?stream=1&stdout=1 HTTP/1.1
    /// Upgrade: tcp
    /// Connection: Upgrade
    /// ```
    ///
    /// The service will respond with a `101 UPGRADED` response, and will
    /// similarly follow with the raw stream:
    ///
    /// ```text
    /// HTTP/1.1 101 UPGRADED
    /// Content-Type: application/vnd.docker.raw-stream
    /// Connection: Upgrade
    /// Upgrade: tcp
    ///
    /// [STREAM]
    /// ```
    ///
    /// ### Stream format
    ///
    /// When the TTY setting is disabled for the container,
    /// the HTTP Content-Type header is set to application/vnd.docker.multiplexed-stream
    /// (starting with v4.7.0, previously application/vnd.docker.raw-stream was always used)
    /// and the stream over the hijacked connected is multiplexed to separate out
    /// `stdout` and `stderr`. The stream consists of a series of frames, each
    /// containing a header and a payload.
    ///
    /// The header contains the information about the output stream type and the size of
    /// the payload.
    /// It is encoded on the first eight bytes like this:
    ///
    /// ```go
    /// header := [8]byte{STREAM_TYPE, 0, 0, 0, SIZE1, SIZE2, SIZE3, SIZE4}
    /// ```
    ///
    /// `STREAM_TYPE` can be:
    ///
    /// - 0: `stdin` (is written on `stdout`)
    /// - 1: `stdout`
    /// - 2: `stderr`
    ///
    /// `SIZE1, SIZE2, SIZE3, SIZE4` are the four bytes of the `uint32` size
    /// encoded as big endian.
    ///
    /// Following the header is the payload, which contains the specified number of
    /// bytes as written in the size.
    ///
    /// The simplest way to implement this protocol is the following:
    ///
    /// 1. Read 8 bytes.
    /// 2. Choose `stdout` or `stderr` depending on the first byte.
    /// 3. Extract the frame size from the last four bytes.
    /// 4. Read the extracted size and output it on the correct output.
    /// 5. Goto 1.
    ///
    /// ### Stream format when using a TTY
    ///
    /// When the TTY setting is enabled for the container,
    /// the stream is not multiplexed. The data exchanged over the hijacked
    /// connection is simply the raw data from the process PTY and client's
    /// `stdin`.
    async fn container_attach_libpod<'a>(
        &self,
        name: &str,
        params: Option<super::super::params::ContainerAttachLibpod<'a>>,
    ) -> Result<(), Error> {
        let mut request_url = url::Url::parse(self.get_config().get_base_path())?;
        let mut request_path = request_url.path().to_owned();
        if request_path.ends_with('/') {
            request_path.pop();
        }
        request_path.push_str("/libpod/containers/{name}/attach");
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
    /// GET /libpod/containers/{name}/changes
    /// Report on changes to container's filesystem; adds, deletes or modifications.
    /// Returns which files in a container's filesystem have been added, deleted, or modified. The Kind of modification can be one of:
    ///
    /// 0: Modified
    /// 1: Added
    /// 2: Deleted
    async fn container_changes_libpod<'a>(
        &self,
        name: &str,
        params: Option<super::super::params::ContainerChangesLibpod<'a>>,
    ) -> Result<(), Error> {
        let mut request_url = url::Url::parse(self.get_config().get_base_path())?;
        let mut request_path = request_url.path().to_owned();
        if request_path.ends_with('/') {
            request_path.pop();
        }
        request_path.push_str("/libpod/containers/{name}/changes");
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
    /// POST /libpod/containers/{name}/checkpoint
    /// Checkpoint a container
    async fn container_checkpoint_libpod(
        &self,
        name: &str,
        params: Option<super::super::params::ContainerCheckpointLibpod>,
    ) -> Result<(), Error> {
        let mut request_url = url::Url::parse(self.get_config().get_base_path())?;
        let mut request_path = request_url.path().to_owned();
        if request_path.ends_with('/') {
            request_path.pop();
        }
        request_path.push_str("/libpod/containers/{name}/checkpoint");
        request_path = request_path.replace("{name}", name);
        request_url.set_path(&request_path);
        let mut req_builder = self.get_config().req_builder("POST")?;
        if let Some(params) = params {
            let mut query_pairs = request_url.query_pairs_mut();
            if let Some(keep) = params.keep {
                query_pairs.append_pair("keep", &keep.to_string());
            }
            if let Some(leave_running) = params.leave_running {
                query_pairs.append_pair("leaveRunning", &leave_running.to_string());
            }
            if let Some(tcp_established) = params.tcp_established {
                query_pairs.append_pair("tcpEstablished", &tcp_established.to_string());
            }
            if let Some(export) = params.export {
                query_pairs.append_pair("export", &export.to_string());
            }
            if let Some(ignore_root_fs) = params.ignore_root_fs {
                query_pairs.append_pair("ignoreRootFS", &ignore_root_fs.to_string());
            }
            if let Some(ignore_volumes) = params.ignore_volumes {
                query_pairs.append_pair("ignoreVolumes", &ignore_volumes.to_string());
            }
            if let Some(pre_checkpoint) = params.pre_checkpoint {
                query_pairs.append_pair("preCheckpoint", &pre_checkpoint.to_string());
            }
            if let Some(with_previous) = params.with_previous {
                query_pairs.append_pair("withPrevious", &with_previous.to_string());
            }
            if let Some(file_locks) = params.file_locks {
                query_pairs.append_pair("fileLocks", &file_locks.to_string());
            }
            if let Some(print_stats) = params.print_stats {
                query_pairs.append_pair("printStats", &print_stats.to_string());
            }
        }
        let hyper_uri: hyper::Uri = request_url.as_str().parse()?;
        req_builder = req_builder.uri(hyper_uri);
        let request = req_builder.body(String::new())?;
        request::execute_request_unit(self.get_config(), request).await
    }
    /// GET /libpod/containers/{name}/exists
    /// Check if container exists
    /// Quick way to determine if a container exists by name or ID
    async fn container_exists_libpod(&self, name: &str) -> Result<(), Error> {
        let mut request_url = url::Url::parse(self.get_config().get_base_path())?;
        let mut request_path = request_url.path().to_owned();
        if request_path.ends_with('/') {
            request_path.pop();
        }
        request_path.push_str("/libpod/containers/{name}/exists");
        request_path = request_path.replace("{name}", name);
        request_url.set_path(&request_path);
        let mut req_builder = self.get_config().req_builder("GET")?;
        let hyper_uri: hyper::Uri = request_url.as_str().parse()?;
        req_builder = req_builder.uri(hyper_uri);
        let request = req_builder.body(String::new())?;
        request::execute_request_unit(self.get_config(), request).await
    }
    /// GET /libpod/containers/{name}/export
    /// Export a container
    /// Export the contents of a container as a tarball.
    async fn container_export_libpod(&self, name: &str) -> Result<(), Error> {
        let mut request_url = url::Url::parse(self.get_config().get_base_path())?;
        let mut request_path = request_url.path().to_owned();
        if request_path.ends_with('/') {
            request_path.pop();
        }
        request_path.push_str("/libpod/containers/{name}/export");
        request_path = request_path.replace("{name}", name);
        request_url.set_path(&request_path);
        let mut req_builder = self.get_config().req_builder("GET")?;
        let hyper_uri: hyper::Uri = request_url.as_str().parse()?;
        req_builder = req_builder.uri(hyper_uri);
        let request = req_builder.body(String::new())?;
        request::execute_request_unit(self.get_config(), request).await
    }
    /// GET /libpod/containers/{name}/healthcheck
    /// Run a container's healthcheck
    /// Execute the defined healthcheck and return information about the results
    async fn container_healthcheck_libpod(&self, name: &str) -> Result<(), Error> {
        let mut request_url = url::Url::parse(self.get_config().get_base_path())?;
        let mut request_path = request_url.path().to_owned();
        if request_path.ends_with('/') {
            request_path.pop();
        }
        request_path.push_str("/libpod/containers/{name}/healthcheck");
        request_path = request_path.replace("{name}", name);
        request_url.set_path(&request_path);
        let mut req_builder = self.get_config().req_builder("GET")?;
        let hyper_uri: hyper::Uri = request_url.as_str().parse()?;
        req_builder = req_builder.uri(hyper_uri);
        let request = req_builder.body(String::new())?;
        request::execute_request_unit(self.get_config(), request).await
    }
    /// POST /libpod/containers/{name}/init
    /// Initialize a container
    /// Performs all tasks necessary for initializing the container but does not start the container.
    async fn container_init_libpod(&self, name: &str) -> Result<(), Error> {
        let mut request_url = url::Url::parse(self.get_config().get_base_path())?;
        let mut request_path = request_url.path().to_owned();
        if request_path.ends_with('/') {
            request_path.pop();
        }
        request_path.push_str("/libpod/containers/{name}/init");
        request_path = request_path.replace("{name}", name);
        request_url.set_path(&request_path);
        let mut req_builder = self.get_config().req_builder("POST")?;
        let hyper_uri: hyper::Uri = request_url.as_str().parse()?;
        req_builder = req_builder.uri(hyper_uri);
        let request = req_builder.body(String::new())?;
        request::execute_request_unit(self.get_config(), request).await
    }
    /// GET /libpod/containers/{name}/json
    /// Inspect container
    /// Return low-level information about a container.
    async fn container_inspect_libpod(
        &self,
        name: &str,
        params: Option<super::super::params::ContainerInspectLibpod>,
    ) -> Result<(), Error> {
        let mut request_url = url::Url::parse(self.get_config().get_base_path())?;
        let mut request_path = request_url.path().to_owned();
        if request_path.ends_with('/') {
            request_path.pop();
        }
        request_path.push_str("/libpod/containers/{name}/json");
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
    /// POST /libpod/containers/{name}/kill
    /// Kill container
    /// send a signal to a container, defaults to killing the container
    async fn container_kill_libpod<'a>(
        &self,
        name: &str,
        params: Option<super::super::params::ContainerKillLibpod<'a>>,
    ) -> Result<(), Error> {
        let mut request_url = url::Url::parse(self.get_config().get_base_path())?;
        let mut request_path = request_url.path().to_owned();
        if request_path.ends_with('/') {
            request_path.pop();
        }
        request_path.push_str("/libpod/containers/{name}/kill");
        request_path = request_path.replace("{name}", name);
        request_url.set_path(&request_path);
        let mut req_builder = self.get_config().req_builder("POST")?;
        if let Some(params) = params {
            let mut query_pairs = request_url.query_pairs_mut();
            if let Some(signal) = params.signal {
                query_pairs.append_pair("signal", signal);
            }
        }
        let hyper_uri: hyper::Uri = request_url.as_str().parse()?;
        req_builder = req_builder.uri(hyper_uri);
        let request = req_builder.body(String::new())?;
        request::execute_request_unit(self.get_config(), request).await
    }
    /// GET /libpod/containers/{name}/logs
    /// Get container logs
    /// Get stdout and stderr logs from a container.
    ///
    /// The stream format is the same as described in the attach endpoint.
    async fn container_logs_libpod<'a>(
        &self,
        name: &str,
        params: Option<super::super::params::ContainerLogsLibpod<'a>>,
    ) -> Result<(), Error> {
        let mut request_url = url::Url::parse(self.get_config().get_base_path())?;
        let mut request_path = request_url.path().to_owned();
        if request_path.ends_with('/') {
            request_path.pop();
        }
        request_path.push_str("/libpod/containers/{name}/logs");
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
    /// POST /libpod/containers/{name}/mount
    /// Mount a container
    /// Mount a container to the filesystem
    async fn container_mount_libpod(&self, name: &str) -> Result<String, Error> {
        let mut request_url = url::Url::parse(self.get_config().get_base_path())?;
        let mut request_path = request_url.path().to_owned();
        if request_path.ends_with('/') {
            request_path.pop();
        }
        request_path.push_str("/libpod/containers/{name}/mount");
        request_path = request_path.replace("{name}", name);
        request_url.set_path(&request_path);
        let mut req_builder = self.get_config().req_builder("POST")?;
        let hyper_uri: hyper::Uri = request_url.as_str().parse()?;
        req_builder = req_builder.uri(hyper_uri);
        let request = req_builder.body(String::new())?;
        request::execute_request_json(self.get_config(), request).await
    }
    /// POST /libpod/containers/{name}/pause
    /// Pause a container
    /// Use the cgroups freezer to suspend all processes in a container.
    async fn container_pause_libpod(&self, name: &str) -> Result<(), Error> {
        let mut request_url = url::Url::parse(self.get_config().get_base_path())?;
        let mut request_path = request_url.path().to_owned();
        if request_path.ends_with('/') {
            request_path.pop();
        }
        request_path.push_str("/libpod/containers/{name}/pause");
        request_path = request_path.replace("{name}", name);
        request_url.set_path(&request_path);
        let mut req_builder = self.get_config().req_builder("POST")?;
        let hyper_uri: hyper::Uri = request_url.as_str().parse()?;
        req_builder = req_builder.uri(hyper_uri);
        let request = req_builder.body(String::new())?;
        request::execute_request_unit(self.get_config(), request).await
    }
    /// POST /libpod/containers/{name}/rename
    /// Rename an existing container
    /// Change the name of an existing container.
    async fn container_rename_libpod<'a>(
        &self,
        name: &str,
        params: Option<super::super::params::ContainerRenameLibpod<'a>>,
    ) -> Result<(), Error> {
        let mut request_url = url::Url::parse(self.get_config().get_base_path())?;
        let mut request_path = request_url.path().to_owned();
        if request_path.ends_with('/') {
            request_path.pop();
        }
        request_path.push_str("/libpod/containers/{name}/rename");
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
    /// POST /libpod/containers/{name}/resize
    /// Resize a container's TTY
    /// Resize the terminal attached to a container (for use with Attach).
    async fn container_resize_libpod(
        &self,
        name: &str,
        params: Option<super::super::params::ContainerResizeLibpod>,
    ) -> Result<(), Error> {
        let mut request_url = url::Url::parse(self.get_config().get_base_path())?;
        let mut request_path = request_url.path().to_owned();
        if request_path.ends_with('/') {
            request_path.pop();
        }
        request_path.push_str("/libpod/containers/{name}/resize");
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
        }
        let hyper_uri: hyper::Uri = request_url.as_str().parse()?;
        req_builder = req_builder.uri(hyper_uri);
        let request = req_builder.body(String::new())?;
        request::execute_request_unit(self.get_config(), request).await
    }
    /// POST /libpod/containers/{name}/restart
    /// Restart a container
    async fn container_restart_libpod(
        &self,
        name: &str,
        params: Option<super::super::params::ContainerRestartLibpod>,
    ) -> Result<(), Error> {
        let mut request_url = url::Url::parse(self.get_config().get_base_path())?;
        let mut request_path = request_url.path().to_owned();
        if request_path.ends_with('/') {
            request_path.pop();
        }
        request_path.push_str("/libpod/containers/{name}/restart");
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
    /// POST /libpod/containers/{name}/restore
    /// Restore a container
    /// Restore a container from a checkpoint.
    async fn container_restore_libpod<'a>(
        &self,
        name: &str,
        params: Option<super::super::params::ContainerRestoreLibpod<'a>>,
    ) -> Result<(), Error> {
        let mut request_url = url::Url::parse(self.get_config().get_base_path())?;
        let mut request_path = request_url.path().to_owned();
        if request_path.ends_with('/') {
            request_path.pop();
        }
        request_path.push_str("/libpod/containers/{name}/restore");
        request_path = request_path.replace("{name}", name);
        request_url.set_path(&request_path);
        let mut req_builder = self.get_config().req_builder("POST")?;
        if let Some(params) = params {
            let mut query_pairs = request_url.query_pairs_mut();
            if let Some(name) = params.name {
                query_pairs.append_pair("name", name);
            }
            if let Some(keep) = params.keep {
                query_pairs.append_pair("keep", &keep.to_string());
            }
            if let Some(tcp_established) = params.tcp_established {
                query_pairs.append_pair("tcpEstablished", &tcp_established.to_string());
            }
            if let Some(import) = params.import {
                query_pairs.append_pair("import", &import.to_string());
            }
            if let Some(ignore_root_fs) = params.ignore_root_fs {
                query_pairs.append_pair("ignoreRootFS", &ignore_root_fs.to_string());
            }
            if let Some(ignore_volumes) = params.ignore_volumes {
                query_pairs.append_pair("ignoreVolumes", &ignore_volumes.to_string());
            }
            if let Some(ignore_static_ip) = params.ignore_static_ip {
                query_pairs.append_pair("ignoreStaticIP", &ignore_static_ip.to_string());
            }
            if let Some(ignore_static_mac) = params.ignore_static_mac {
                query_pairs.append_pair("ignoreStaticMAC", &ignore_static_mac.to_string());
            }
            if let Some(file_locks) = params.file_locks {
                query_pairs.append_pair("fileLocks", &file_locks.to_string());
            }
            if let Some(print_stats) = params.print_stats {
                query_pairs.append_pair("printStats", &print_stats.to_string());
            }
            if let Some(pod) = params.pod {
                query_pairs.append_pair("pod", pod);
            }
        }
        let hyper_uri: hyper::Uri = request_url.as_str().parse()?;
        req_builder = req_builder.uri(hyper_uri);
        let request = req_builder.body(String::new())?;
        request::execute_request_unit(self.get_config(), request).await
    }
    /// POST /libpod/containers/{name}/start
    /// Start a container
    async fn container_start_libpod<'a>(
        &self,
        name: &str,
        params: Option<super::super::params::ContainerStartLibpod<'a>>,
    ) -> Result<(), Error> {
        let mut request_url = url::Url::parse(self.get_config().get_base_path())?;
        let mut request_path = request_url.path().to_owned();
        if request_path.ends_with('/') {
            request_path.pop();
        }
        request_path.push_str("/libpod/containers/{name}/start");
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
    /// GET /libpod/containers/{name}/stats
    /// Get stats for a container
    /// DEPRECATED. This endpoint will be removed with the next major release. Please use /libpod/containers/stats instead.
    async fn container_stats_libpod(
        &self,
        name: &str,
        params: Option<super::super::params::ContainerStatsLibpod>,
    ) -> Result<(), Error> {
        let mut request_url = url::Url::parse(self.get_config().get_base_path())?;
        let mut request_path = request_url.path().to_owned();
        if request_path.ends_with('/') {
            request_path.pop();
        }
        request_path.push_str("/libpod/containers/{name}/stats");
        request_path = request_path.replace("{name}", name);
        request_url.set_path(&request_path);
        let mut req_builder = self.get_config().req_builder("GET")?;
        if let Some(params) = params {
            let mut query_pairs = request_url.query_pairs_mut();
            if let Some(stream) = params.stream {
                query_pairs.append_pair("stream", &stream.to_string());
            }
        }
        let hyper_uri: hyper::Uri = request_url.as_str().parse()?;
        req_builder = req_builder.uri(hyper_uri);
        let request = req_builder.body(String::new())?;
        request::execute_request_unit(self.get_config(), request).await
    }
    /// POST /libpod/containers/{name}/stop
    /// Stop a container
    async fn container_stop_libpod(
        &self,
        name: &str,
        params: Option<super::super::params::ContainerStopLibpod>,
    ) -> Result<(), Error> {
        let mut request_url = url::Url::parse(self.get_config().get_base_path())?;
        let mut request_path = request_url.path().to_owned();
        if request_path.ends_with('/') {
            request_path.pop();
        }
        request_path.push_str("/libpod/containers/{name}/stop");
        request_path = request_path.replace("{name}", name);
        request_url.set_path(&request_path);
        let mut req_builder = self.get_config().req_builder("POST")?;
        if let Some(params) = params {
            let mut query_pairs = request_url.query_pairs_mut();
            if let Some(timeout) = params.timeout {
                query_pairs.append_pair("timeout", &timeout.to_string());
            }
            if let Some(ignore) = params.ignore {
                query_pairs.append_pair("Ignore", &ignore.to_string());
            }
        }
        let hyper_uri: hyper::Uri = request_url.as_str().parse()?;
        req_builder = req_builder.uri(hyper_uri);
        let request = req_builder.body(String::new())?;
        request::execute_request_unit(self.get_config(), request).await
    }
    /// GET /libpod/containers/{name}/top
    /// List processes
    /// List processes running inside a container
    async fn container_top_libpod<'a>(
        &self,
        name: &str,
        params: Option<super::super::params::ContainerTopLibpod<'a>>,
    ) -> Result<(), Error> {
        let mut request_url = url::Url::parse(self.get_config().get_base_path())?;
        let mut request_path = request_url.path().to_owned();
        if request_path.ends_with('/') {
            request_path.pop();
        }
        request_path.push_str("/libpod/containers/{name}/top");
        request_path = request_path.replace("{name}", name);
        request_url.set_path(&request_path);
        let mut req_builder = self.get_config().req_builder("GET")?;
        if let Some(params) = params {
            let mut query_pairs = request_url.query_pairs_mut();
            if let Some(stream) = params.stream {
                query_pairs.append_pair("stream", &stream.to_string());
            }
            if let Some(delay) = params.delay {
                query_pairs.append_pair("delay", &delay.to_string());
            }
            if let Some(ps_args) = params.ps_args {
                query_pairs.append_pair(
                    "ps_args",
                    &ps_args
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
        request::execute_request_unit(self.get_config(), request).await
    }
    /// POST /libpod/containers/{name}/unmount
    /// Unmount a container
    /// Unmount a container from the filesystem
    async fn container_unmount_libpod(&self, name: &str) -> Result<(), Error> {
        let mut request_url = url::Url::parse(self.get_config().get_base_path())?;
        let mut request_path = request_url.path().to_owned();
        if request_path.ends_with('/') {
            request_path.pop();
        }
        request_path.push_str("/libpod/containers/{name}/unmount");
        request_path = request_path.replace("{name}", name);
        request_url.set_path(&request_path);
        let mut req_builder = self.get_config().req_builder("POST")?;
        let hyper_uri: hyper::Uri = request_url.as_str().parse()?;
        req_builder = req_builder.uri(hyper_uri);
        let request = req_builder.body(String::new())?;
        request::execute_request_unit(self.get_config(), request).await
    }
    /// POST /libpod/containers/{name}/unpause
    /// Unpause Container
    async fn container_unpause_libpod(&self, name: &str) -> Result<(), Error> {
        let mut request_url = url::Url::parse(self.get_config().get_base_path())?;
        let mut request_path = request_url.path().to_owned();
        if request_path.ends_with('/') {
            request_path.pop();
        }
        request_path.push_str("/libpod/containers/{name}/unpause");
        request_path = request_path.replace("{name}", name);
        request_url.set_path(&request_path);
        let mut req_builder = self.get_config().req_builder("POST")?;
        let hyper_uri: hyper::Uri = request_url.as_str().parse()?;
        req_builder = req_builder.uri(hyper_uri);
        let request = req_builder.body(String::new())?;
        request::execute_request_unit(self.get_config(), request).await
    }
    /// POST /libpod/containers/{name}/update
    /// Update an existing containers cgroup configuration
    /// Update an existing containers cgroup configuration.
    async fn container_update_libpod(&self, name: &str, resources: ()) -> Result<(), Error> {
        let mut request_url = url::Url::parse(self.get_config().get_base_path())?;
        let mut request_path = request_url.path().to_owned();
        if request_path.ends_with('/') {
            request_path.pop();
        }
        request_path.push_str("/libpod/containers/{name}/update");
        request_path = request_path.replace("{name}", name);
        request_url.set_path(&request_path);
        let mut req_builder = self.get_config().req_builder("POST")?;
        let hyper_uri: hyper::Uri = request_url.as_str().parse()?;
        req_builder = req_builder.uri(hyper_uri);
        let body = serde_json::to_string(&resources)?;
        req_builder = req_builder.header(hyper::header::CONTENT_TYPE, "application/json");
        req_builder = req_builder.header(hyper::header::CONTENT_LENGTH, body.len());
        let request = req_builder.body(body)?;
        request::execute_request_unit(self.get_config(), request).await
    }
    /// POST /libpod/containers/{name}/wait
    /// Wait on a container
    /// Wait on a container to meet a given condition
    async fn container_wait_libpod<'a>(
        &self,
        name: &str,
        params: Option<super::super::params::ContainerWaitLibpod<'a>>,
    ) -> Result<i32, Error> {
        let mut request_url = url::Url::parse(self.get_config().get_base_path())?;
        let mut request_path = request_url.path().to_owned();
        if request_path.ends_with('/') {
            request_path.pop();
        }
        request_path.push_str("/libpod/containers/{name}/wait");
        request_path = request_path.replace("{name}", name);
        request_url.set_path(&request_path);
        let mut req_builder = self.get_config().req_builder("POST")?;
        if let Some(params) = params {
            let mut query_pairs = request_url.query_pairs_mut();
            if let Some(condition) = params.condition {
                query_pairs.append_pair(
                    "condition",
                    &condition
                        .iter()
                        .map(|e| e.to_string())
                        .collect::<Vec<_>>()
                        .join(","),
                );
            }
            if let Some(interval) = params.interval {
                query_pairs.append_pair("interval", interval);
            }
        }
        let hyper_uri: hyper::Uri = request_url.as_str().parse()?;
        req_builder = req_builder.uri(hyper_uri);
        let request = req_builder.body(String::new())?;
        request::execute_request_json(self.get_config(), request).await
    }
    /// POST /libpod/containers/create
    /// Create a container
    async fn container_create_libpod(&self, create: ()) -> Result<(), Error> {
        let mut request_url = url::Url::parse(self.get_config().get_base_path())?;
        let mut request_path = request_url.path().to_owned();
        if request_path.ends_with('/') {
            request_path.pop();
        }
        request_path.push_str("/libpod/containers/create");
        request_url.set_path(&request_path);
        let mut req_builder = self.get_config().req_builder("POST")?;
        let hyper_uri: hyper::Uri = request_url.as_str().parse()?;
        req_builder = req_builder.uri(hyper_uri);
        let body = serde_json::to_string(&create)?;
        req_builder = req_builder.header(hyper::header::CONTENT_TYPE, "application/json");
        req_builder = req_builder.header(hyper::header::CONTENT_LENGTH, body.len());
        let request = req_builder.body(body)?;
        request::execute_request_unit(self.get_config(), request).await
    }
    /// GET /libpod/containers/json
    /// List containers
    /// Returns a list of containers
    async fn container_list_libpod<'a>(
        &self,
        params: Option<super::super::params::ContainerListLibpod<'a>>,
    ) -> Result<Vec<()>, Error> {
        let mut request_url = url::Url::parse(self.get_config().get_base_path())?;
        let mut request_path = request_url.path().to_owned();
        if request_path.ends_with('/') {
            request_path.pop();
        }
        request_path.push_str("/libpod/containers/json");
        request_url.set_path(&request_path);
        let mut req_builder = self.get_config().req_builder("GET")?;
        if let Some(params) = params {
            let mut query_pairs = request_url.query_pairs_mut();
            if let Some(all) = params.all {
                query_pairs.append_pair("all", &all.to_string());
            }
            if let Some(limit) = params.limit {
                query_pairs.append_pair("limit", &limit.to_string());
            }
            if let Some(namespace) = params.namespace {
                query_pairs.append_pair("namespace", &namespace.to_string());
            }
            if let Some(pod) = params.pod {
                query_pairs.append_pair("pod", &pod.to_string());
            }
            if let Some(size) = params.size {
                query_pairs.append_pair("size", &size.to_string());
            }
            if let Some(sync) = params.sync {
                query_pairs.append_pair("sync", &sync.to_string());
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
    /// POST /libpod/containers/prune
    /// Delete stopped containers
    /// Remove containers not in use
    async fn container_prune_libpod<'a>(
        &self,
        params: Option<super::super::params::ContainerPruneLibpod<'a>>,
    ) -> Result<Vec<()>, Error> {
        let mut request_url = url::Url::parse(self.get_config().get_base_path())?;
        let mut request_path = request_url.path().to_owned();
        if request_path.ends_with('/') {
            request_path.pop();
        }
        request_path.push_str("/libpod/containers/prune");
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
    /// GET /libpod/containers/showmounted
    /// Show mounted containers
    /// Lists all mounted containers mount points
    async fn container_show_mounted_libpod(
        &self,
    ) -> Result<std::collections::HashMap<String, String>, Error> {
        let mut request_url = url::Url::parse(self.get_config().get_base_path())?;
        let mut request_path = request_url.path().to_owned();
        if request_path.ends_with('/') {
            request_path.pop();
        }
        request_path.push_str("/libpod/containers/showmounted");
        request_url.set_path(&request_path);
        let mut req_builder = self.get_config().req_builder("GET")?;
        let hyper_uri: hyper::Uri = request_url.as_str().parse()?;
        req_builder = req_builder.uri(hyper_uri);
        let request = req_builder.body(String::new())?;
        request::execute_request_json(self.get_config(), request).await
    }
    /// GET /libpod/containers/stats
    /// Get stats for one or more containers
    /// Return a live stream of resource usage statistics of one or more container. If no container is specified, the statistics of all containers are returned.
    async fn containers_stats_all_libpod<'a>(
        &self,
        params: Option<super::super::params::ContainersStatsAllLibpod<'a>>,
    ) -> Result<(), Error> {
        let mut request_url = url::Url::parse(self.get_config().get_base_path())?;
        let mut request_path = request_url.path().to_owned();
        if request_path.ends_with('/') {
            request_path.pop();
        }
        request_path.push_str("/libpod/containers/stats");
        request_url.set_path(&request_path);
        let mut req_builder = self.get_config().req_builder("GET")?;
        if let Some(params) = params {
            let mut query_pairs = request_url.query_pairs_mut();
            if let Some(containers) = params.containers {
                query_pairs.append_pair(
                    "containers",
                    &containers
                        .iter()
                        .map(|e| e.to_string())
                        .collect::<Vec<_>>()
                        .join(","),
                );
            }
            if let Some(stream) = params.stream {
                query_pairs.append_pair("stream", &stream.to_string());
            }
            if let Some(interval) = params.interval {
                query_pairs.append_pair("interval", &interval.to_string());
            }
        }
        let hyper_uri: hyper::Uri = request_url.as_str().parse()?;
        req_builder = req_builder.uri(hyper_uri);
        let request = req_builder.body(String::new())?;
        request::execute_request_unit(self.get_config(), request).await
    }
    /// GET /libpod/generate/{name}/systemd
    /// Generate Systemd Units
    /// Generate Systemd Units based on a pod or container.
    async fn generate_systemd_libpod<'a>(
        &self,
        name: &str,
        params: Option<super::super::params::GenerateSystemdLibpod<'a>>,
    ) -> Result<std::collections::HashMap<String, String>, Error> {
        let mut request_url = url::Url::parse(self.get_config().get_base_path())?;
        let mut request_path = request_url.path().to_owned();
        if request_path.ends_with('/') {
            request_path.pop();
        }
        request_path.push_str("/libpod/generate/{name}/systemd");
        request_path = request_path.replace("{name}", name);
        request_url.set_path(&request_path);
        let mut req_builder = self.get_config().req_builder("GET")?;
        if let Some(params) = params {
            let mut query_pairs = request_url.query_pairs_mut();
            if let Some(use_name) = params.use_name {
                query_pairs.append_pair("useName", &use_name.to_string());
            }
            if let Some(new) = params.new {
                query_pairs.append_pair("new", &new.to_string());
            }
            if let Some(no_header) = params.no_header {
                query_pairs.append_pair("noHeader", &no_header.to_string());
            }
            if let Some(start_timeout) = params.start_timeout {
                query_pairs.append_pair("startTimeout", &start_timeout.to_string());
            }
            if let Some(stop_timeout) = params.stop_timeout {
                query_pairs.append_pair("stopTimeout", &stop_timeout.to_string());
            }
            if let Some(restart_policy) = params.restart_policy {
                query_pairs.append_pair("restartPolicy", restart_policy);
            }
            if let Some(container_prefix) = params.container_prefix {
                query_pairs.append_pair("containerPrefix", container_prefix);
            }
            if let Some(pod_prefix) = params.pod_prefix {
                query_pairs.append_pair("podPrefix", pod_prefix);
            }
            if let Some(separator) = params.separator {
                query_pairs.append_pair("separator", separator);
            }
            if let Some(restart_sec) = params.restart_sec {
                query_pairs.append_pair("restartSec", &restart_sec.to_string());
            }
            if let Some(wants) = params.wants {
                query_pairs.append_pair(
                    "wants",
                    &wants
                        .iter()
                        .map(|e| e.to_string())
                        .collect::<Vec<_>>()
                        .join(","),
                );
            }
            if let Some(after) = params.after {
                query_pairs.append_pair(
                    "after",
                    &after
                        .iter()
                        .map(|e| e.to_string())
                        .collect::<Vec<_>>()
                        .join(","),
                );
            }
            if let Some(requires) = params.requires {
                query_pairs.append_pair(
                    "requires",
                    &requires
                        .iter()
                        .map(|e| e.to_string())
                        .collect::<Vec<_>>()
                        .join(","),
                );
            }
            if let Some(additional_env_variables) = params.additional_env_variables {
                query_pairs.append_pair(
                    "additionalEnvVariables",
                    &additional_env_variables
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
    /// GET /libpod/generate/kube
    /// Generate a Kubernetes YAML file.
    /// Generate Kubernetes YAML based on a pod or container.
    async fn generate_kube_libpod<'a>(
        &self,
        params: Option<super::super::params::GenerateKubeLibpod<'a>>,
    ) -> Result<String, Error> {
        let mut request_url = url::Url::parse(self.get_config().get_base_path())?;
        let mut request_path = request_url.path().to_owned();
        if request_path.ends_with('/') {
            request_path.pop();
        }
        request_path.push_str("/libpod/generate/kube");
        request_url.set_path(&request_path);
        let mut req_builder = self.get_config().req_builder("GET")?;
        if let Some(params) = params {
            let mut query_pairs = request_url.query_pairs_mut();
            query_pairs.append_pair(
                "names",
                &params
                    .names
                    .iter()
                    .map(|e| e.to_string())
                    .collect::<Vec<_>>()
                    .join(","),
            );
            if let Some(service) = params.service {
                query_pairs.append_pair("service", &service.to_string());
            }
            if let Some(r#type) = params.r#type {
                query_pairs.append_pair("type", r#type);
            }
            if let Some(replicas) = params.replicas {
                query_pairs.append_pair("replicas", &replicas.to_string());
            }
            if let Some(no_trunc) = params.no_trunc {
                query_pairs.append_pair("noTrunc", &no_trunc.to_string());
            }
            if let Some(podman_only) = params.podman_only {
                query_pairs.append_pair("podmanOnly", &podman_only.to_string());
            }
        }
        let hyper_uri: hyper::Uri = request_url.as_str().parse()?;
        req_builder = req_builder.uri(hyper_uri);
        let request = req_builder.body(String::new())?;
        request::execute_request_json(self.get_config(), request).await
    }
    /// POST /libpod/kube/apply
    /// Apply a podman workload or Kubernetes YAML file.
    /// Deploy a podman container, pod, volume, or Kubernetes yaml to a Kubernetes cluster.
    async fn kube_apply_libpod<'a>(
        &self,
        params: Option<super::super::params::KubeApplyLibpod<'a>>,
        request: String,
    ) -> Result<String, Error> {
        let mut request_url = url::Url::parse(self.get_config().get_base_path())?;
        let mut request_path = request_url.path().to_owned();
        if request_path.ends_with('/') {
            request_path.pop();
        }
        request_path.push_str("/libpod/kube/apply");
        request_url.set_path(&request_path);
        let mut req_builder = self.get_config().req_builder("POST")?;
        if let Some(params) = params {
            let mut query_pairs = request_url.query_pairs_mut();
            if let Some(ca_cert_file) = params.ca_cert_file {
                query_pairs.append_pair("caCertFile", ca_cert_file);
            }
            if let Some(kube_config) = params.kube_config {
                query_pairs.append_pair("kubeConfig", kube_config);
            }
            if let Some(namespace) = params.namespace {
                query_pairs.append_pair("namespace", namespace);
            }
            if let Some(service) = params.service {
                query_pairs.append_pair("service", &service.to_string());
            }
            if let Some(file) = params.file {
                query_pairs.append_pair("file", file);
            }
        }
        let hyper_uri: hyper::Uri = request_url.as_str().parse()?;
        req_builder = req_builder.uri(hyper_uri);
        let body = serde_json::to_string(&request)?;
        req_builder = req_builder.header(hyper::header::CONTENT_TYPE, "application/json");
        req_builder = req_builder.header(hyper::header::CONTENT_LENGTH, body.len());
        let request = req_builder.body(body)?;
        request::execute_request_json(self.get_config(), request).await
    }
    /// DELETE /libpod/play/kube
    /// Remove resources created from kube play
    /// Tears down pods, secrets, and volumes defined in a YAML file
    async fn play_kube_down_libpod(
        &self,
        params: Option<super::super::params::PlayKubeDownLibpod>,
    ) -> Result<(), Error> {
        let mut request_url = url::Url::parse(self.get_config().get_base_path())?;
        let mut request_path = request_url.path().to_owned();
        if request_path.ends_with('/') {
            request_path.pop();
        }
        request_path.push_str("/libpod/play/kube");
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
    /// POST /libpod/play/kube
    /// Play a Kubernetes YAML file.
    /// Create and run pods based on a Kubernetes YAML file (pod or service kind).
    async fn play_kube_libpod<'a>(
        &self,
        params: Option<super::super::params::PlayKubeLibpod<'a>>,
        request: String,
    ) -> Result<(), Error> {
        let mut request_url = url::Url::parse(self.get_config().get_base_path())?;
        let mut request_path = request_url.path().to_owned();
        if request_path.ends_with('/') {
            request_path.pop();
        }
        request_path.push_str("/libpod/play/kube");
        request_url.set_path(&request_path);
        let mut req_builder = self.get_config().req_builder("POST")?;
        if let Some(params) = params {
            let mut query_pairs = request_url.query_pairs_mut();
            if let Some(log_driver) = params.log_driver {
                query_pairs.append_pair("logDriver", log_driver);
            }
            if let Some(log_options) = params.log_options {
                query_pairs.append_pair(
                    "logOptions",
                    &log_options
                        .iter()
                        .map(|e| e.to_string())
                        .collect::<Vec<_>>()
                        .join(","),
                );
            }
            if let Some(network) = params.network {
                query_pairs.append_pair(
                    "network",
                    &network
                        .iter()
                        .map(|e| e.to_string())
                        .collect::<Vec<_>>()
                        .join(","),
                );
            }
            if let Some(no_hosts) = params.no_hosts {
                query_pairs.append_pair("noHosts", &no_hosts.to_string());
            }
            if let Some(no_trunc) = params.no_trunc {
                query_pairs.append_pair("noTrunc", &no_trunc.to_string());
            }
            if let Some(publish_ports) = params.publish_ports {
                query_pairs.append_pair(
                    "publishPorts",
                    &publish_ports
                        .iter()
                        .map(|e| e.to_string())
                        .collect::<Vec<_>>()
                        .join(","),
                );
            }
            if let Some(replace) = params.replace {
                query_pairs.append_pair("replace", &replace.to_string());
            }
            if let Some(service_container) = params.service_container {
                query_pairs.append_pair("serviceContainer", &service_container.to_string());
            }
            if let Some(start) = params.start {
                query_pairs.append_pair("start", &start.to_string());
            }
            if let Some(static_i_ps) = params.static_i_ps {
                query_pairs.append_pair(
                    "staticIPs",
                    &static_i_ps
                        .iter()
                        .map(|e| e.to_string())
                        .collect::<Vec<_>>()
                        .join(","),
                );
            }
            if let Some(static_ma_cs) = params.static_ma_cs {
                query_pairs.append_pair(
                    "staticMACs",
                    &static_ma_cs
                        .iter()
                        .map(|e| e.to_string())
                        .collect::<Vec<_>>()
                        .join(","),
                );
            }
            if let Some(tls_verify) = params.tls_verify {
                query_pairs.append_pair("tlsVerify", &tls_verify.to_string());
            }
            if let Some(userns) = params.userns {
                query_pairs.append_pair("userns", userns);
            }
            if let Some(wait) = params.wait {
                query_pairs.append_pair("wait", &wait.to_string());
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
}

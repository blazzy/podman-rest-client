use crate::v4::config::HasConfig;
use crate::v4::request;
use crate::v4::Error;
#[async_trait::async_trait]
pub trait ExecCompat: HasConfig + Send + Sync {
    /// POST /containers/{name}/exec
    /// Create an exec instance
    /// Create an exec session to run a command inside a running container. Exec sessions will be automatically removed 5 minutes after they exit.
    async fn container_exec(
        &self,
        name: &str,
        control: super::super::models::ContainerExecBody,
    ) -> Result<(), Error> {
        let mut request_url = url::Url::parse(self.get_config().get_base_path())?;
        let mut request_path = request_url.path().to_owned();
        if request_path.ends_with('/') {
            request_path.pop();
        }
        request_path.push_str("/containers/{name}/exec");
        request_path = request_path.replace("{name}", name);
        request_url.set_path(&request_path);
        let mut req_builder = self.get_config().req_builder("POST")?;
        let hyper_uri: hyper::Uri = request_url.as_str().parse()?;
        req_builder = req_builder.uri(hyper_uri);
        let body = serde_json::to_string(&control)?;
        req_builder = req_builder.header(hyper::header::CONTENT_TYPE, "application/json");
        req_builder = req_builder.header(hyper::header::CONTENT_LENGTH, body.len());
        let request = req_builder.body(body)?;
        request::execute_request_unit(self.get_config(), request).await
    }
    /// GET /exec/{id}/json
    /// Inspect an exec instance
    /// Return low-level information about an exec instance.
    async fn exec_inspect(&self, id: &str) -> Result<(), Error> {
        let mut request_url = url::Url::parse(self.get_config().get_base_path())?;
        let mut request_path = request_url.path().to_owned();
        if request_path.ends_with('/') {
            request_path.pop();
        }
        request_path.push_str("/exec/{id}/json");
        request_path = request_path.replace("{id}", id);
        request_url.set_path(&request_path);
        let mut req_builder = self.get_config().req_builder("GET")?;
        let hyper_uri: hyper::Uri = request_url.as_str().parse()?;
        req_builder = req_builder.uri(hyper_uri);
        let request = req_builder.body(String::new())?;
        request::execute_request_unit(self.get_config(), request).await
    }
    /// POST /exec/{id}/resize
    /// Resize an exec instance
    /// Resize the TTY session used by an exec instance. This endpoint only works if tty was specified as part of creating and starting the exec instance.
    async fn exec_resize(
        &self,
        id: &str,
        params: Option<super::super::params::ExecResize>,
    ) -> Result<(), Error> {
        let mut request_url = url::Url::parse(self.get_config().get_base_path())?;
        let mut request_path = request_url.path().to_owned();
        if request_path.ends_with('/') {
            request_path.pop();
        }
        request_path.push_str("/exec/{id}/resize");
        request_path = request_path.replace("{id}", id);
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
    /// POST /exec/{id}/start
    /// Start an exec instance
    /// Starts a previously set up exec instance. If detach is true, this endpoint returns immediately after starting the command. Otherwise, it sets up an interactive session with the command.
    async fn exec_start(
        &self,
        id: &str,
        control: super::super::models::ExecStartBody,
    ) -> Result<(), Error> {
        let mut request_url = url::Url::parse(self.get_config().get_base_path())?;
        let mut request_path = request_url.path().to_owned();
        if request_path.ends_with('/') {
            request_path.pop();
        }
        request_path.push_str("/exec/{id}/start");
        request_path = request_path.replace("{id}", id);
        request_url.set_path(&request_path);
        let mut req_builder = self.get_config().req_builder("POST")?;
        let hyper_uri: hyper::Uri = request_url.as_str().parse()?;
        req_builder = req_builder.uri(hyper_uri);
        let body = serde_json::to_string(&control)?;
        req_builder = req_builder.header(hyper::header::CONTENT_TYPE, "application/json");
        req_builder = req_builder.header(hyper::header::CONTENT_LENGTH, body.len());
        let request = req_builder.body(body)?;
        request::execute_request_unit(self.get_config(), request).await
    }
}

use crate::api_common::config::HasConfig;
use crate::api_common::request;
use crate::api_common::Error;
use http::request::Builder;
use std::future::Future;
use std::pin::Pin;
pub trait Manifests: HasConfig + Send + Sync {
    /// DELETE /libpod/manifests/{name}
    ///
    /// Delete manifest list
    ///
    /// Delete named manifest list
    ///
    /// As of v4.0.0
    fn manifest_delete_libpod<'a>(
        &'a self,
        name: &'a str,
    ) -> Pin<
        Box<
            dyn Future<Output = Result<crate::v5::models::LibpodImagesRemoveReport, Error>>
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
                request_path.push_str("/libpod/manifests/{name}");
                request_path = request_path.replace("{name}", name);
                request_url.set_path(&request_path);
                let hyper_uri: hyper::Uri = request_url.as_str().parse()?;
                req_builder = req_builder.uri(hyper_uri);
                Ok(req_builder.body(String::new())?)
            },
        ))
    }
    /// POST /libpod/manifests/{name}
    ///
    /// Create
    ///
    /// Create a manifest list
    fn manifest_create_libpod<'a>(
        &'a self,
        name: &'a str,
        params: Option<crate::v5::params::ManifestCreateLibpod<'a>>,
        options: crate::v5::models::ManifestModifyOptions,
    ) -> Pin<Box<dyn Future<Output = Result<crate::v5::models::IdResponse, Error>> + Send + 'a>>
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
                request_path.push_str("/libpod/manifests/{name}");
                request_path = request_path.replace("{name}", name);
                request_url.set_path(&request_path);
                if let Some(params) = &params {
                    let mut query_pairs = request_url.query_pairs_mut();
                    query_pairs.append_pair("images", params.images);
                    if let Some(all) = params.all {
                        query_pairs.append_pair("all", &all.to_string());
                    }
                    if let Some(amend) = params.amend {
                        query_pairs.append_pair("amend", &amend.to_string());
                    }
                }
                let hyper_uri: hyper::Uri = request_url.as_str().parse()?;
                req_builder = req_builder.uri(hyper_uri);
                let body = serde_json::to_string(&options)?;
                req_builder = req_builder.header(hyper::header::CONTENT_TYPE, "application/json");
                req_builder = req_builder.header(hyper::header::CONTENT_LENGTH, body.len());
                Ok(req_builder.body(body)?)
            },
        ))
    }
    /// GET /libpod/manifests/{name}
    ///
    /// Modify manifest list
    ///
    /// Add/Remove an image(s) to a manifest list
    ///
    /// Note: operations are not atomic when multiple Images are provided.
    ///
    /// As of v4.0.0
    fn manifest_modify_libpod<'a>(
        &'a self,
        name: &'a str,
        params: Option<crate::v5::params::ManifestModifyLibpod>,
        options: crate::v5::models::ManifestModifyOptions,
    ) -> Pin<
        Box<
            dyn Future<Output = Result<crate::v5::models::ManifestModifyReport, Error>> + Send + 'a,
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
                request_path.push_str("/libpod/manifests/{name}");
                request_path = request_path.replace("{name}", name);
                request_url.set_path(&request_path);
                if let Some(params) = &params {
                    let mut query_pairs = request_url.query_pairs_mut();
                    if let Some(tls_verify) = params.tls_verify {
                        query_pairs.append_pair("tlsVerify", &tls_verify.to_string());
                    }
                }
                let hyper_uri: hyper::Uri = request_url.as_str().parse()?;
                req_builder = req_builder.uri(hyper_uri);
                let body = serde_json::to_string(&options)?;
                req_builder = req_builder.header(hyper::header::CONTENT_TYPE, "application/json");
                req_builder = req_builder.header(hyper::header::CONTENT_LENGTH, body.len());
                Ok(req_builder.body(body)?)
            },
        ))
    }
    /// POST /libpod/manifests/{name}/add
    ///
    /// Add image
    ///
    /// Add an image to a manifest list
    ///
    /// Deprecated: As of 4.0.0 use ManifestModifyLibpod instead
    fn manifest_add_libpod<'a>(
        &'a self,
        name: &'a str,
        options: crate::v5::models::ManifestAddOptions,
    ) -> Pin<Box<dyn Future<Output = Result<crate::v5::models::IdResponse, Error>> + Send + 'a>>
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
                request_path.push_str("/libpod/manifests/{name}/add");
                request_path = request_path.replace("{name}", name);
                request_url.set_path(&request_path);
                let hyper_uri: hyper::Uri = request_url.as_str().parse()?;
                req_builder = req_builder.uri(hyper_uri);
                let body = serde_json::to_string(&options)?;
                req_builder = req_builder.header(hyper::header::CONTENT_TYPE, "application/json");
                req_builder = req_builder.header(hyper::header::CONTENT_LENGTH, body.len());
                Ok(req_builder.body(body)?)
            },
        ))
    }
    /// GET /libpod/manifests/{name}/exists
    ///
    /// Exists
    ///
    /// Check if manifest list exists
    ///
    /// Note: There is no contract that the manifest list will exist for a follow-on operation
    fn manifest_exists_libpod<'a>(
        &'a self,
        name: &'a str,
    ) -> Pin<Box<dyn Future<Output = Result<(), Error>> + Send + 'a>> {
        Box::pin(request::execute_request_unit(
            self.get_config(),
            move |mut req_builder: Builder| {
                req_builder = req_builder.method("GET");
                let mut request_url = url::Url::parse(self.get_config().get_base_path())?;
                let mut request_path = request_url.path().to_owned();
                if request_path.ends_with('/') {
                    request_path.pop();
                }
                request_path.push_str("/libpod/manifests/{name}/exists");
                request_path = request_path.replace("{name}", name);
                request_url.set_path(&request_path);
                let hyper_uri: hyper::Uri = request_url.as_str().parse()?;
                req_builder = req_builder.uri(hyper_uri);
                Ok(req_builder.body(String::new())?)
            },
        ))
    }
    /// GET /libpod/manifests/{name}/json
    ///
    /// Inspect
    ///
    /// Display attributes of given manifest list
    fn manifest_inspect_libpod<'a>(
        &'a self,
        name: &'a str,
        params: Option<crate::v5::params::ManifestInspectLibpod>,
    ) -> Pin<
        Box<dyn Future<Output = Result<crate::v5::models::Schema2ListPublic, Error>> + Send + 'a>,
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
                request_path.push_str("/libpod/manifests/{name}/json");
                request_path = request_path.replace("{name}", name);
                request_url.set_path(&request_path);
                if let Some(params) = &params {
                    let mut query_pairs = request_url.query_pairs_mut();
                    if let Some(tls_verify) = params.tls_verify {
                        query_pairs.append_pair("tlsVerify", &tls_verify.to_string());
                    }
                }
                let hyper_uri: hyper::Uri = request_url.as_str().parse()?;
                req_builder = req_builder.uri(hyper_uri);
                Ok(req_builder.body(String::new())?)
            },
        ))
    }
    /// POST /libpod/manifests/{name}/push
    ///
    /// Push manifest to registry
    ///
    /// Push a manifest list or image index to a registry
    ///
    /// Deprecated: As of 4.0.0 use ManifestPushLibpod instead
    fn manifest_push_v_3_libpod<'a>(
        &'a self,
        name: &'a str,
        params: Option<crate::v5::params::ManifestPushV3Libpod<'a>>,
    ) -> Pin<Box<dyn Future<Output = Result<crate::v5::models::IdResponse, Error>> + Send + 'a>>
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
                request_path.push_str("/libpod/manifests/{name}/push");
                request_path = request_path.replace("{name}", name);
                request_url.set_path(&request_path);
                if let Some(params) = &params {
                    let mut query_pairs = request_url.query_pairs_mut();
                    query_pairs.append_pair("destination", params.destination);
                    if let Some(all) = params.all {
                        query_pairs.append_pair("all", &all.to_string());
                    }
                }
                let hyper_uri: hyper::Uri = request_url.as_str().parse()?;
                req_builder = req_builder.uri(hyper_uri);
                Ok(req_builder.body(String::new())?)
            },
        ))
    }
    /// POST /libpod/manifests/{name}/registry/{destination}
    ///
    /// Push manifest list to registry
    ///
    /// Push a manifest list or image index to the named registry
    ///
    /// As of v4.0.0
    fn manifest_push_libpod<'a>(
        &'a self,
        name: &'a str,
        destination: &'a str,
        params: Option<crate::v5::params::ManifestPushLibpod<'a>>,
    ) -> Pin<Box<dyn Future<Output = Result<crate::v5::models::IdResponse, Error>> + Send + 'a>>
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
                request_path.push_str("/libpod/manifests/{name}/registry/{destination}");
                request_path = request_path.replace("{name}", name);
                request_path = request_path.replace("{destination}", destination);
                request_url.set_path(&request_path);
                if let Some(params) = &params {
                    let mut query_pairs = request_url.query_pairs_mut();
                    if let Some(add_compression) = &params.add_compression {
                        for value in add_compression {
                            query_pairs.append_pair("addCompression", &value.to_string());
                        }
                    }
                    if let Some(force_compression_format) = params.force_compression_format {
                        query_pairs.append_pair(
                            "forceCompressionFormat",
                            &force_compression_format.to_string(),
                        );
                    }
                    if let Some(all) = params.all {
                        query_pairs.append_pair("all", &all.to_string());
                    }
                    if let Some(tls_verify) = params.tls_verify {
                        query_pairs.append_pair("tlsVerify", &tls_verify.to_string());
                    }
                    if let Some(quiet) = params.quiet {
                        query_pairs.append_pair("quiet", &quiet.to_string());
                    }
                }
                let hyper_uri: hyper::Uri = request_url.as_str().parse()?;
                req_builder = req_builder.uri(hyper_uri);
                Ok(req_builder.body(String::new())?)
            },
        ))
    }
}

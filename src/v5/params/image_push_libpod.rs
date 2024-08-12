#[derive(Default, Debug)]
pub struct ImagePushLibpod<'a> {
    /// A base64-encoded auth configuration.
    pub x_registry_auth: Option<&'a str>,
    /// Allows for pushing the image to a different destination than the image refers to.
    pub destination: Option<&'a str>,
    /// Enforce compressing the layers with the specified --compression and do not reuse differently compressed blobs on the registry.
    pub force_compression_format: Option<bool>,
    /// Require TLS verification.
    pub tls_verify: Option<bool>,
    /// silences extra stream data on push
    pub quiet: Option<bool>,
}

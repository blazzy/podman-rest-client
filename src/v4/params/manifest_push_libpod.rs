#[derive(Default, Debug)]
pub struct ManifestPushLibpod<'a> {
    /// add existing instances with requested compression algorithms to manifest list
    pub add_compression: Option<Vec<&'a str>>,
    /// Enforce compressing the layers with the specified --compression and do not reuse differently compressed blobs on the registry.
    pub force_compression_format: Option<bool>,
    /// push all images
    pub all: Option<bool>,
    /// Require HTTPS and verify signatures when contacting registries.
    pub tls_verify: Option<bool>,
    /// silences extra stream data on push
    pub quiet: Option<bool>,
}

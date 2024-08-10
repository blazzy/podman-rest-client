#[derive(Default, Debug)]
pub struct ManifestModifyLibpod {
    /// Require HTTPS and verify signatures when contacting registries.
    pub tls_verify: Option<bool>,
}

#[derive(Default, Debug)]
pub struct ManifestInspectLibpod {
    /// Require HTTPS and verify signatures when contacting registries.
    pub tls_verify: Option<bool>,
}

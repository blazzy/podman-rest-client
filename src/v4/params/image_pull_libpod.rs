#[derive(Default, Debug)]
pub struct ImagePullLibpod<'a> {
    /// base-64 encoded auth config. Must include the following four values: username, password, email and server address OR simply just an identity token.
    pub x_registry_auth: Option<&'a str>,
    /// Mandatory reference to the image (e.g., quay.io/image/name:tag)
    pub reference: Option<&'a str>,
    /// silences extra stream data on pull
    pub quiet: Option<bool>,
    /// Return the same JSON payload as the Docker-compat endpoint.
    pub compat_mode: Option<bool>,
    /// Pull image for the specified architecture.
    pub arch: Option<&'a str>,
    /// Pull image for the specified operating system.
    pub os: Option<&'a str>,
    /// Pull image for the specified variant.
    pub variant: Option<&'a str>,
    /// Pull policy, "always" (default), "missing", "newer", "never".
    pub policy: Option<&'a str>,
    /// Require TLS verification.
    pub tls_verify: Option<bool>,
    /// Pull all tagged images in the repository.
    pub all_tags: Option<bool>,
}

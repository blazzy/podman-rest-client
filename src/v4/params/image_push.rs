#[derive(Default, Debug)]
pub struct ImagePush<'a> {
    /// A base64-encoded auth configuration.
    pub x_registry_auth: Option<&'a str>,
    /// The tag to associate with the image on the registry.
    pub tag: Option<&'a str>,
    /// All indicates whether to push all images related to the image list
    pub all: Option<bool>,
    /// use compression on image
    pub compress: Option<bool>,
    /// destination name for the image being pushed
    pub destination: Option<&'a str>,
}

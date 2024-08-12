#[derive(Default, Debug)]
pub struct ImageCreate<'a> {
    /// A base64-encoded auth configuration.
    pub x_registry_auth: Option<&'a str>,
    /// Name of the image to pull. The name may include a tag or digest. This parameter may only be used when pulling an image. The pull is cancelled if the HTTP connection is closed.
    pub from_image: Option<&'a str>,
    /// Source to import. The value may be a URL from which the image can be retrieved or - to read the image from the request body. This parameter may only be used when importing an image
    pub from_src: Option<&'a str>,
    /// Repository name given to an image when it is imported. The repo may include a tag. This parameter may only be used when importing an image.
    pub repo: Option<&'a str>,
    /// Tag or digest. If empty when pulling an image, this causes all tags for the given image to be pulled.
    pub tag: Option<&'a str>,
    /// Set commit message for imported image.
    pub message: Option<&'a str>,
    /// Platform in the format os[/arch[/variant]]
    pub platform: Option<&'a str>,
}

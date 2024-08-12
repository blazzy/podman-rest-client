#[derive(Default, Debug)]
pub struct ImageImportLibpod<'a> {
    pub content_type: Option<&'a str>,
    /// Apply the following possible instructions to the created image: CMD | ENTRYPOINT | ENV | EXPOSE | LABEL | STOPSIGNAL | USER | VOLUME | WORKDIR.  JSON encoded string
    pub changes: Option<Vec<&'a str>>,
    /// Set commit message for imported image
    pub message: Option<&'a str>,
    /// Optional Name[:TAG] for the image
    pub reference: Option<&'a str>,
    /// Load image from the specified URL
    pub url: Option<&'a str>,
}

#[derive(Default, Debug)]
pub struct ImageExportLibpod<'a> {
    /// format for exported image (only docker-archive is supported)
    pub format: Option<&'a str>,
    /// references to images to export
    pub references: Option<Vec<&'a str>>,
    /// use compression on image
    pub compress: Option<bool>,
    /// accept uncompressed layers when copying OCI images
    pub oci_accept_uncompressed_layers: Option<bool>,
}

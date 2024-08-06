#[derive(Default, Debug)]
pub struct ManifestPushV3Libpod<'a> {
    /// the destination for the manifest
    pub destination: &'a str,

    /// push all images
    pub all: Option<bool>,
}

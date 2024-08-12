#[derive(Default, Debug)]
pub struct ImageDeleteAllLibpod<'a> {
    /// Images IDs or names to remove.
    pub images: Option<Vec<&'a str>>,
    /// Remove all images.
    pub all: Option<bool>,
    /// Force image removal (including containers using the images).
    pub force: Option<bool>,
    /// Ignore if a specified image does not exist and do not throw an error.
    pub ignore: Option<bool>,
    /// Resolves to manifest list instead of image.
    pub lookup_manifest: Option<bool>,
}

#[derive(Default, Debug)]
pub struct ManifestCreateLibpod<'a> {
    /// manifest list or index name to create
    pub name: &'a str,
    /// One or more names of an image or a manifest list. Repeat parameter as needed.
    ///
    /// Support for multiple images, as of version 4.0.0
    /// Alias of `image` is support for compatibility with < 4.0.0
    /// Response status code is 200 with < 4.0.0 for compatibility
    pub images: &'a str,
    /// add all contents if given list
    pub all: Option<bool>,
    /// modify an existing list if one with the desired name already exists
    pub amend: Option<bool>,
}

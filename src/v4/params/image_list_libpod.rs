#[derive(Default, Debug)]
pub struct ImageListLibpod<'a> {
    /// Show all images. Only images from a final layer (no children) are shown by default.
    pub all: Option<bool>,
    /// A JSON encoded value of the filters (a `map[string][]string`) to process on the images list. Available filters:
    /// - `before`=(`<image-name>[:<tag>]`,  `<image id>` or `<image@digest>`)
    /// - `dangling=true`
    /// - `label=key` or `label="key=value"` of an image label
    /// - `reference`=(`<image-name>[:<tag>]`)
    /// - `id`=(`<image-id>`)
    /// - `since`=(`<image-name>[:<tag>]`,  `<image id>` or `<image@digest>`)
    pub filters: Option<&'a str>,
}

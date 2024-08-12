#[derive(Default, Debug)]
pub struct ImageCommit<'a> {
    /// the name or ID of a container
    pub container: Option<&'a str>,
    /// the repository name for the created image
    pub repo: Option<&'a str>,
    /// tag name for the created image
    pub tag: Option<&'a str>,
    /// commit message
    pub comment: Option<&'a str>,
    /// author of the image
    pub author: Option<&'a str>,
    /// pause the container before committing it
    pub pause: Option<bool>,
    /// instructions to apply while committing in Dockerfile format
    pub changes: Option<&'a str>,
    /// squash newly built layers into a single new layer
    pub squash: Option<bool>,
}

#[derive(Default, Debug)]
pub struct ImageCommitLibpod<'a> {
    /// the name or ID of a container
    pub container: &'a str,
    /// author of the image
    pub author: Option<&'a str>,
    /// instructions to apply while committing in Dockerfile format (i.e. "CMD=/bin/foo")
    pub changes: Option<Vec<&'a str>>,
    /// commit message
    pub comment: Option<&'a str>,
    /// format of the image manifest and metadata (default "oci")
    pub format: Option<&'a str>,
    /// pause the container before committing it
    pub pause: Option<bool>,
    /// squash the container before committing it
    pub squash: Option<bool>,
    /// the repository name for the created image
    pub repo: Option<&'a str>,
    /// output from commit process
    pub stream: Option<bool>,
    /// tag name for the created image
    pub tag: Option<&'a str>,
}

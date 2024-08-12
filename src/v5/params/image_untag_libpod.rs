#[derive(Default, Debug)]
pub struct ImageUntagLibpod<'a> {
    /// the repository to untag
    pub repo: Option<&'a str>,
    /// the name of the tag to untag
    pub tag: Option<&'a str>,
}

#[derive(Default, Debug)]
pub struct ImageTagLibpod<'a> {
    /// the repository to tag in
    pub repo: Option<&'a str>,

    /// the name of the new tag
    pub tag: Option<&'a str>,
}

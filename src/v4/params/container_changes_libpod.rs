#[derive(Default, Debug)]
pub struct ContainerChangesLibpod<'a> {
    /// specify a second layer which is used to compare against it instead of the parent layer
    pub parent: Option<&'a str>,
    /// select what you want to match, default is all
    pub diff_type: Option<&'a str>,
}

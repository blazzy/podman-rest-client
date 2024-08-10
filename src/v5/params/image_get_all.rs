#[derive(Default, Debug)]
pub struct ImageGetAll<'a> {
    /// one or more image names or IDs comma separated
    pub names: &'a str,
}

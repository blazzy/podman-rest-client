#[derive(Default, Debug)]
pub struct ImageTreeLibpod {
    /// show all child images and layers of the specified image
    pub whatrequires: Option<bool>,
}

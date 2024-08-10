#[derive(Default, Debug)]
pub struct VolumeDelete {
    /// Force removal of the volume. This actually only causes errors due
    /// to the names volume not being found to be suppressed, which is the
    /// behaviour Docker implements.
    pub force: Option<bool>,
}

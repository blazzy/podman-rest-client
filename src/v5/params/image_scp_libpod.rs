#[derive(Default, Debug)]
pub struct ImageScpLibpod<'a> {
    /// dest connection/image
    pub destination: Option<&'a str>,

    /// quiet output
    pub quiet: Option<bool>,
}

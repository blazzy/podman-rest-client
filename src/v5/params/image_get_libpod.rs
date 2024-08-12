#[derive(Default, Debug)]
pub struct ImageGetLibpod<'a> {
    /// format for exported image
    pub format: Option<&'a str>,
    /// use compression on image
    pub compress: Option<bool>,
}

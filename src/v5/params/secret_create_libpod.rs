#[derive(Default, Debug)]
pub struct SecretCreateLibpod<'a> {
    /// User-defined name of the secret.
    pub name: &'a str,
    /// Secret driver
    pub driver: Option<&'a str>,
    /// Secret driver options
    pub driveropts: Option<&'a str>,
    /// Labels on the secret
    pub labels: Option<&'a str>,
}

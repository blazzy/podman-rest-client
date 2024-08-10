#[derive(Default, Debug)]
pub struct ContainerStartLibpod<'a> {
    /// Override the key sequence for detaching a container. Format is a single character [a-Z] or ctrl-<value> where <value> is one of: a-z, @, ^, [, , or _.
    pub detach_keys: Option<&'a str>,
}

#[derive(Default, Debug)]
pub struct NetworkInspect<'a> {
    /// Detailed inspect output for troubleshooting
    pub verbose: Option<bool>,
    /// Filter the network by scope (swarm, global, or local)
    pub scope: Option<&'a str>,
}

#[derive(Default, Debug)]
pub struct ContainerStats {
    /// Stream the output
    pub stream: Option<bool>,

    /// Provide a one-shot response in which preCPU stats are blank, resulting in a single cycle return.
    pub one_shot: Option<bool>,
}

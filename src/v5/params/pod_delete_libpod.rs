#[derive(Default, Debug)]
pub struct PodDeleteLibpod {
    /// force removal of a running pod by first stopping all containers, then removing all containers in the pod
    pub force: Option<bool>,
}

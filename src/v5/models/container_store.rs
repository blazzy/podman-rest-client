use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Serialize, Deserialize)]
/// ContainerStore describes the quantity of containers in the
/// store by status
pub struct ContainerStore {
    pub number: Option<i64>,

    pub paused: Option<i64>,

    pub running: Option<i64>,

    pub stopped: Option<i64>,
}

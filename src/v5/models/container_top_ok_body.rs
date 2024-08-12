use serde::{Deserialize, Serialize};
#[derive(Default, Debug, Serialize, Deserialize)]
/// ContainerTopOKBody OK response to ContainerTop operation
pub struct ContainerTopOkBody {
    /// Each process running in the container, where each is process
    /// is an array of values corresponding to the titles.
    #[serde(rename = "Processes")]
    pub processes: Vec<Vec<String>>,
    /// The ps column titles
    #[serde(rename = "Titles")]
    pub titles: Vec<String>,
}

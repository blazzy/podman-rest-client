use serde::{Deserialize, Serialize};
#[derive(Default, Debug, Serialize, Deserialize)]
/// Ulimit is a human friendly version of Rlimit.
pub struct Ulimit {
    #[serde(rename = "Hard")]
    pub hard: Option<i64>,
    #[serde(rename = "Name")]
    pub name: Option<String>,
    #[serde(rename = "Soft")]
    pub soft: Option<i64>,
}

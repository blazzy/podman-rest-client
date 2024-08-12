use serde::{Deserialize, Serialize};
#[derive(Default, Debug, Serialize, Deserialize)]
/// Namespace describes the namespace
pub struct Namespace {
    pub nsmode: Option<String>,
    pub value: Option<String>,
}

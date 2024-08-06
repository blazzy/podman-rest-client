use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Serialize, Deserialize)]
/// WaitExitError container waiting error, if any
pub struct WaitExitError {
    /// Details of an error
    #[serde(rename = "Message")]
    pub message: Option<String>,
}

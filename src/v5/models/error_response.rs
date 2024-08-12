use serde::{Deserialize, Serialize};
#[derive(Default, Debug, Serialize, Deserialize)]
/// ErrorResponse Represents an error.
pub struct ErrorResponse {
    /// The error message.
    pub message: String,
}

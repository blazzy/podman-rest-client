use serde::{Deserialize, Serialize};
#[derive(Default, Debug, Serialize, Deserialize)]
pub struct SecretInUse {
    /// API root cause formatted for automated parsing
    pub cause: Option<String>,
    /// human error message, formatted for a human to read
    pub message: Option<String>,
    /// HTTP response code
    pub response: Option<i64>,
}

use serde::{Deserialize, Serialize};
#[derive(Default, Debug, Serialize, Deserialize)]
pub struct ExecStartLibpodBody {
    /// Detach from the command.
    #[serde(rename = "Detach")]
    pub detach: Option<bool>,
    /// Allocate a pseudo-TTY.
    #[serde(rename = "Tty")]
    pub tty: Option<bool>,
    /// Height of the TTY session in characters. Tty must be set to true to use it.
    pub h: Option<i64>,
    /// Width of the TTY session in characters. Tty must be set to true to use it.
    pub w: Option<i64>,
}

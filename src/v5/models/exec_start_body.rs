use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Serialize, Deserialize)]
pub struct ExecStartBody {
    /// Detach from the command. Not presently supported.
    #[serde(rename = "Detach")]
    pub detach: Option<bool>,

    /// Allocate a pseudo-TTY. Presently ignored.
    #[serde(rename = "Tty")]
    pub tty: Option<bool>,
}

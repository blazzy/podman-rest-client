use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Serialize, Deserialize)]
pub struct LibpodContainersRmReport {
    /// Error which occurred during Rm operation (if any).
    /// This field is optional and may be omitted if no error occurred.
    #[serde(rename = "Err")]
    pub err: Option<String>,

    #[serde(rename = "Id")]
    pub id: Option<String>,
}

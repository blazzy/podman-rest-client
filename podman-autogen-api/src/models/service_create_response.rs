use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Serialize, Deserialize)]
/// ServiceCreateResponse contains the information returned to a client on the
/// creation of a new service.
pub struct ServiceCreateResponse {
    /// The ID of the created service.
    #[serde(rename = "ID")]
    pub id: Option<String>,

    /// Optional warning message.
    ///
    /// FIXME(thaJeztah): this should have "omitempty" in the generated type.
    #[serde(rename = "Warnings")]
    pub warnings: Option<Vec<String>>,
}

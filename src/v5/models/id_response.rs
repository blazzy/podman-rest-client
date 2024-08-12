use serde::{Deserialize, Serialize};
#[derive(Default, Debug, Serialize, Deserialize)]
/// IDResponse Response to an API call that returns just an Id
pub struct IdResponse {
    /// The id of the newly created object.
    #[serde(rename = "Id")]
    pub id: String,
}

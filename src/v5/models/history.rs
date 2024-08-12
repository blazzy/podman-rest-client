use serde::{Deserialize, Serialize};
#[derive(Default, Debug, Serialize, Deserialize)]
/// History describes the history of a layer.
pub struct History {
    /// Author is the author of the build point.
    pub author: Option<String>,
    /// Comment is a custom message set when creating the layer.
    pub comment: Option<String>,
    /// Created is the combined date and time at which the layer was created, formatted as defined by RFC 3339, section 5.6.
    pub created: Option<String>,
    /// CreatedBy is the command which created the layer.
    pub created_by: Option<String>,
    /// EmptyLayer is used to mark if the history item created a filesystem diff.
    pub empty_layer: Option<bool>,
}

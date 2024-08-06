use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Serialize, Deserialize)]
/// HistoryResponseItem individual image layer information in response to ImageHistory operation
pub struct HistoryResponseItem {
    /// comment
    #[serde(rename = "Comment")]
    pub comment: String,

    /// created
    #[serde(rename = "Created")]
    pub created: i64,

    /// created by
    #[serde(rename = "CreatedBy")]
    pub created_by: String,

    /// Id
    #[serde(rename = "Id")]
    pub id: String,

    /// size
    #[serde(rename = "Size")]
    pub size: i64,

    /// tags
    #[serde(rename = "Tags")]
    pub tags: Vec<String>,
}

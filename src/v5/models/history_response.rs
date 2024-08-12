use serde::{Deserialize, Serialize};
#[derive(Default, Debug, Serialize, Deserialize)]
/// HistoryResponse provides details on image layers
pub struct HistoryResponse {
    #[serde(rename = "Comment")]
    pub comment: Option<String>,
    #[serde(rename = "Created")]
    pub created: Option<i64>,
    #[serde(rename = "CreatedBy")]
    pub created_by: Option<String>,
    #[serde(rename = "Id")]
    pub id: Option<String>,
    #[serde(rename = "Size")]
    pub size: Option<i64>,
    #[serde(rename = "Tags")]
    pub tags: Option<Vec<String>>,
}

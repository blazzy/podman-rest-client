use serde::{Deserialize, Serialize};
#[derive(Default, Debug, Serialize, Deserialize)]
/// SystemDfImageReport describes an image for use with df
pub struct SystemDfImageReport {
    #[serde(rename = "Containers")]
    pub containers: Option<i64>,
    #[serde(rename = "Created")]
    pub created: Option<String>,
    #[serde(rename = "ImageID")]
    pub image_id: Option<String>,
    #[serde(rename = "Repository")]
    pub repository: Option<String>,
    #[serde(rename = "SharedSize")]
    pub shared_size: Option<i64>,
    #[serde(rename = "Size")]
    pub size: Option<i64>,
    #[serde(rename = "Tag")]
    pub tag: Option<String>,
    #[serde(rename = "UniqueSize")]
    pub unique_size: Option<i64>,
}

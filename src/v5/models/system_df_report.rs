use serde::{Deserialize, Serialize};
#[derive(Default, Debug, Serialize, Deserialize)]
/// SystemDfReport describes the response for df information
pub struct SystemDfReport {
    #[serde(rename = "Containers")]
    pub containers: Option<Vec<crate::v5::models::SystemDfContainerReport>>,
    #[serde(rename = "Images")]
    pub images: Option<Vec<crate::v5::models::SystemDfImageReport>>,
    #[serde(rename = "ImagesSize")]
    pub images_size: Option<i64>,
    #[serde(rename = "Volumes")]
    pub volumes: Option<Vec<crate::v5::models::SystemDfVolumeReport>>,
}

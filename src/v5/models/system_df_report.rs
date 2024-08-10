use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Serialize, Deserialize)]
/// SystemDfReport describes the response for df information
pub struct SystemDfReport {
    #[serde(rename = "Containers")]
    pub containers: Option<Vec<super::super::models::SystemDfContainerReport>>,

    #[serde(rename = "Images")]
    pub images: Option<Vec<super::super::models::SystemDfImageReport>>,

    #[serde(rename = "ImagesSize")]
    pub images_size: Option<i64>,

    #[serde(rename = "Volumes")]
    pub volumes: Option<Vec<super::super::models::SystemDfVolumeReport>>,
}

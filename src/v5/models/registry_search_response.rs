use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Serialize, Deserialize)]
pub struct RegistrySearchResponse {
    /// Automated indicates if the image was created by an automated build.
    #[serde(rename = "Automated")]
    pub automated: Option<String>,

    /// Description of the image.
    #[serde(rename = "Description")]
    pub description: Option<String>,

    /// Index is the image index
    #[serde(rename = "Index")]
    pub index: Option<String>,

    /// Name is the canonical name of the image
    #[serde(rename = "Name")]
    pub name: Option<String>,

    /// Official indicates if it's an official image.
    #[serde(rename = "Official")]
    pub official: Option<String>,

    /// Stars is the number of stars of the image.
    #[serde(rename = "Stars")]
    pub stars: Option<i64>,

    /// Tag is the image tag
    #[serde(rename = "Tag")]
    pub tag: Option<String>,
}

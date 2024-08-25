use serde::{Deserialize, Serialize};
#[derive(Default, Debug, Serialize, Deserialize)]
/// ManifestAnnotateOptions provides model for annotating manifest list
pub struct ManifestAnnotateOptions {
    /// Annotation to add to the item in the manifest list
    pub annotation: Option<Vec<String>>,
    /// Annotations to add to the item in the manifest list by a map which is preferred over Annotation
    pub annotations: Option<std::collections::HashMap<String, String>>,
    /// Arch overrides the architecture for the item in the manifest list
    pub arch: Option<String>,
    /// Feature list for the item in the manifest list
    pub features: Option<Vec<String>>,
    /// IndexAnnotation is a slice of key=value annotations to add to the manifest list itself
    pub index_annotation: Option<Vec<String>>,
    /// IndexAnnotations is a map of key:value annotations to add to the manifest list itself, by a map which is preferred over IndexAnnotation
    pub index_annotations: Option<std::collections::HashMap<String, String>>,
    /// OS overrides the operating system for the item in the manifest list
    pub os: Option<String>,
    /// OS features for the item in the manifest list
    pub os_features: Option<Vec<String>>,
    /// OSVersion overrides the operating system for the item in the manifest list
    pub os_version: Option<String>,
    /// IndexSubject is a subject value to set in the manifest list itself
    pub subject: Option<String>,
    /// Variant for the item in the manifest list
    pub variant: Option<String>,
}

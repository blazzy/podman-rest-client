use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Serialize, Deserialize)]
/// Summary summary
pub struct Summary {
    /// Number of containers using this image. Includes both stopped and running
    /// containers.
    ///
    /// This size is not calculated by default, and depends on which API endpoint
    /// is used. `-1` indicates that the value has not been set / calculated.
    #[serde(rename = "Containers")]
    pub containers: i64,

    /// Date and time at which the image was created as a Unix timestamp
    /// (number of seconds sinds EPOCH).
    #[serde(rename = "Created")]
    pub created: i64,

    /// ID is the content-addressable ID of an image.
    ///
    /// This identifier is a content-addressable digest calculated from the
    /// image's configuration (which includes the digests of layers used by
    /// the image).
    ///
    /// Note that this digest differs from the `RepoDigests` below, which
    /// holds digests of image manifests that reference the image.
    #[serde(rename = "Id")]
    pub id: String,

    /// User-defined key/value metadata.
    #[serde(rename = "Labels")]
    pub labels: std::collections::HashMap<String, String>,

    /// ID of the parent image.
    ///
    /// Depending on how the image was created, this field may be empty and
    /// is only set for images that were built/created locally. This field
    /// is empty if the image was pulled from an image registry.
    #[serde(rename = "ParentId")]
    pub parent_id: String,

    /// List of content-addressable digests of locally available image manifests
    /// that the image is referenced from. Multiple manifests can refer to the
    /// same image.
    ///
    /// These digests are usually only available if the image was either pulled
    /// from a registry, or if the image was pushed to a registry, which is when
    /// the manifest is generated and its digest calculated.
    #[serde(rename = "RepoDigests")]
    pub repo_digests: Vec<String>,

    /// List of image names/tags in the local image cache that reference this
    /// image.
    ///
    /// Multiple image tags can refer to the same image, and this list may be
    /// empty if no tags reference the image, in which case the image is
    /// "untagged", in which case it can still be referenced by its ID.
    #[serde(rename = "RepoTags")]
    pub repo_tags: Vec<String>,

    /// Total size of image layers that are shared between this image and other
    /// images.
    ///
    /// This size is not calculated by default. `-1` indicates that the value
    /// has not been set / calculated.
    #[serde(rename = "SharedSize")]
    pub shared_size: i64,

    /// Total size of the image including all layers it is composed of.
    #[serde(rename = "Size")]
    pub size: i64,

    /// Total size of the image including all layers it is composed of.
    ///
    /// Deprecated: this field is omitted in API v1.44, but kept for backward compatibility. Use Size instead.
    #[serde(rename = "VirtualSize")]
    pub virtual_size: Option<i64>,
}

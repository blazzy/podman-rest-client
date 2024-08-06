use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Serialize, Deserialize)]
pub struct ImageInspect {
    /// Architecture is the hardware CPU architecture that the image runs on.
    #[serde(rename = "Architecture")]
    pub architecture: Option<String>,

    /// Author is the name of the author that was specified when committing the
    /// image, or as specified through MAINTAINER (deprecated) in the Dockerfile.
    #[serde(rename = "Author")]
    pub author: Option<String>,

    /// Comment is an optional message that can be set when committing or
    /// importing the image.
    #[serde(rename = "Comment")]
    pub comment: Option<String>,

    #[serde(rename = "Config")]
    pub config: Option<super::super::models::Config>,

    /// Container is for backwards compat but is basically unused
    #[serde(rename = "Container")]
    pub container: Option<String>,

    #[serde(rename = "ContainerConfig")]
    pub container_config: Option<super::super::models::Config>,

    /// Created is the date and time at which the image was created, formatted in
    /// RFC 3339 nano-seconds (time.RFC3339Nano).
    ///
    /// This information is only available if present in the image,
    /// and omitted otherwise.
    #[serde(rename = "Created")]
    pub created: Option<String>,

    /// DockerVersion is the version of Docker that was used to build the image.
    ///
    /// Depending on how the image was created, this field may be empty.
    #[serde(rename = "DockerVersion")]
    pub docker_version: Option<String>,

    #[serde(rename = "GraphDriver")]
    pub graph_driver: Option<super::super::models::GraphDriverData>,

    /// ID is the content-addressable ID of an image.
    ///
    /// This identifier is a content-addressable digest calculated from the
    /// image's configuration (which includes the digests of layers used by
    /// the image).
    ///
    /// Note that this digest differs from the `RepoDigests` below, which
    /// holds digests of image manifests that reference the image.
    #[serde(rename = "Id")]
    pub id: Option<String>,

    #[serde(rename = "Metadata")]
    pub metadata: Option<super::super::models::Metadata>,

    /// OS is the Operating System the image is built to run on.
    #[serde(rename = "Os")]
    pub os: Option<String>,

    /// OsVersion is the version of the Operating System the image is built to
    /// run on (especially for Windows).
    #[serde(rename = "OsVersion")]
    pub os_version: Option<String>,

    /// Parent is the ID of the parent image.
    ///
    /// Depending on how the image was created, this field may be empty and
    /// is only set for images that were built/created locally. This field
    /// is empty if the image was pulled from an image registry.
    #[serde(rename = "Parent")]
    pub parent: Option<String>,

    /// RepoDigests is a list of content-addressable digests of locally available
    /// image manifests that the image is referenced from. Multiple manifests can
    /// refer to the same image.
    ///
    /// These digests are usually only available if the image was either pulled
    /// from a registry, or if the image was pushed to a registry, which is when
    /// the manifest is generated and its digest calculated.
    #[serde(rename = "RepoDigests")]
    pub repo_digests: Option<Vec<String>>,

    /// RepoTags is a list of image names/tags in the local image cache that
    /// reference this image.
    ///
    /// Multiple image tags can refer to the same image, and this list may be
    /// empty if no tags reference the image, in which case the image is
    /// "untagged", in which case it can still be referenced by its ID.
    #[serde(rename = "RepoTags")]
    pub repo_tags: Option<Vec<String>>,

    #[serde(rename = "RootFS")]
    pub root_fs: Option<super::super::models::RootFs>,

    /// Size is the total size of the image including all layers it is composed of.
    #[serde(rename = "Size")]
    pub size: Option<i64>,

    /// Variant is the CPU architecture variant (presently ARM-only).
    #[serde(rename = "Variant")]
    pub variant: Option<String>,

    /// VirtualSize is the total size of the image including all layers it is
    /// composed of.
    ///
    /// Deprecated: this field is omitted in API v1.44, but kept for backward compatibility. Use Size instead.
    #[serde(rename = "VirtualSize")]
    pub virtual_size: Option<i64>,
}

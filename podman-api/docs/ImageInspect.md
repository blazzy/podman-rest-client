# ImageInspect

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**architecture** | Option<**String**> | Architecture is the hardware CPU architecture that the image runs on. | [optional]
**author** | Option<**String**> | Author is the name of the author that was specified when committing the image, or as specified through MAINTAINER (deprecated) in the Dockerfile. | [optional]
**comment** | Option<**String**> | Comment is an optional message that can be set when committing or importing the image. | [optional]
**config** | Option<[**models::Config**](Config.md)> |  | [optional]
**container** | Option<**String**> | Container is for backwards compat but is basically unused | [optional]
**container_config** | Option<[**models::Config**](Config.md)> |  | [optional]
**created** | Option<**String**> | Created is the date and time at which the image was created, formatted in RFC 3339 nano-seconds (time.RFC3339Nano).  This information is only available if present in the image, and omitted otherwise. | [optional]
**docker_version** | Option<**String**> | DockerVersion is the version of Docker that was used to build the image.  Depending on how the image was created, this field may be empty. | [optional]
**graph_driver** | Option<[**models::GraphDriverData**](GraphDriverData.md)> |  | [optional]
**id** | Option<**String**> | ID is the content-addressable ID of an image.  This identifier is a content-addressable digest calculated from the image's configuration (which includes the digests of layers used by the image).  Note that this digest differs from the `RepoDigests` below, which holds digests of image manifests that reference the image. | [optional]
**metadata** | Option<[**models::Metadata**](Metadata.md)> |  | [optional]
**os** | Option<**String**> | OS is the Operating System the image is built to run on. | [optional]
**os_version** | Option<**String**> | OsVersion is the version of the Operating System the image is built to run on (especially for Windows). | [optional]
**parent** | Option<**String**> | Parent is the ID of the parent image.  Depending on how the image was created, this field may be empty and is only set for images that were built/created locally. This field is empty if the image was pulled from an image registry. | [optional]
**repo_digests** | Option<**Vec<String>**> | RepoDigests is a list of content-addressable digests of locally available image manifests that the image is referenced from. Multiple manifests can refer to the same image.  These digests are usually only available if the image was either pulled from a registry, or if the image was pushed to a registry, which is when the manifest is generated and its digest calculated. | [optional]
**repo_tags** | Option<**Vec<String>**> | RepoTags is a list of image names/tags in the local image cache that reference this image.  Multiple image tags can refer to the same image, and this list may be empty if no tags reference the image, in which case the image is \"untagged\", in which case it can still be referenced by its ID. | [optional]
**root_fs** | Option<[**models::RootFs**](RootFS.md)> |  | [optional]
**size** | Option<**i64**> | Size is the total size of the image including all layers it is composed of. | [optional]
**variant** | Option<**String**> | Variant is the CPU architecture variant (presently ARM-only). | [optional]
**virtual_size** | Option<**i64**> | VirtualSize is the total size of the image including all layers it is composed of.  Deprecated: this field is omitted in API v1.44, but kept for backward compatibility. Use Size instead. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)



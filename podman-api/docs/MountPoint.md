# MountPoint

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**destination** | Option<**String**> | Destination is the path relative to the container root (`/`) where the Source is mounted inside the container. | [optional]
**driver** | Option<**String**> | Driver is the volume driver used to create the volume (if it is a volume). | [optional]
**mode** | Option<**String**> | Mode is a comma separated list of options supplied by the user when creating the bind/volume mount.  The default is platform-specific (`\"z\"` on Linux, empty on Windows). | [optional]
**name** | Option<**String**> | Name is the name reference to the underlying data defined by `Source` e.g., the volume name. | [optional]
**propagation** | Option<**String**> |  | [optional]
**rw** | Option<**bool**> | RW indicates whether the mount is mounted writable (read-write). | [optional]
**source** | Option<**String**> | Source is the source location of the mount.  For volumes, this contains the storage location of the volume (within `/var/lib/docker/volumes/`). For bind-mounts, and `npipe`, this contains the source (host) part of the bind-mount. For `tmpfs` mount points, this field is empty. | [optional]
**r#type** | Option<**String**> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)



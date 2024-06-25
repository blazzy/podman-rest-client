# VolumeConfigResponse

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**anonymous** | Option<**bool**> | Anonymous indicates that the volume was created as an anonymous volume for a specific container, and will be removed when any container using it is removed. | [optional]
**created_at** | Option<**String**> | CreatedAt is the date and time the volume was created at. This is not stored for older Libpod volumes; if so, it will be omitted. | [optional]
**driver** | Option<**String**> | Driver is the driver used to create the volume. If set to \"local\" or \"\", the Local driver (Podman built-in code) is used to service the volume; otherwise, a volume plugin with the given name is used to mount and manage the volume. | [optional]
**gid** | Option<**i64**> | GID is the GID that the volume was created with. | [optional]
**labels** | Option<**std::collections::HashMap<String, String>**> | Labels includes the volume's configured labels, key:value pairs that can be passed during volume creation to provide information for third party tools. | [optional]
**lock_number** | Option<**i32**> | LockNumber is the number of the volume's Libpod lock. | [optional]
**mount_count** | Option<**i32**> | MountCount is the number of times this volume has been mounted. | [optional]
**mountpoint** | Option<**String**> | Mountpoint is the path on the host where the volume is mounted. | [optional]
**name** | Option<**String**> | Name is the name of the volume. | [optional]
**needs_chown** | Option<**bool**> | NeedsChown indicates that the next time the volume is mounted into a container, the container will chown the volume to the container process UID/GID. | [optional]
**needs_copy_up** | Option<**bool**> | NeedsCopyUp indicates that the next time the volume is mounted into | [optional]
**options** | Option<**std::collections::HashMap<String, String>**> | Options is a set of options that were used when creating the volume. For the Local driver, these are mount options that will be used to determine how a local filesystem is mounted; they are handled as parameters to Mount in a manner described in the volume create manpage. For non-local drivers, these are passed as-is to the volume plugin. | [optional]
**scope** | Option<**String**> | Scope is unused and provided solely for Docker compatibility. It is unconditionally set to \"local\". | [optional]
**status** | Option<[**std::collections::HashMap<String, serde_json::Value>**](serde_json::Value.md)> | Status is used to return information on the volume's current state, if the volume was created using a volume plugin (uses a Driver that is not the local driver). Status is provided to us by an external program, so no guarantees are made about its format or contents. Further, it is an optional field, so it may not be set even in cases where a volume plugin is in use. | [optional]
**storage_id** | Option<**String**> | StorageID is the ID of the container backing the volume in c/storage. Only used with Image Volumes. | [optional]
**timeout** | Option<**i32**> | Timeout is the specified driver timeout if given | [optional]
**uid** | Option<**i64**> | UID is the UID that the volume was created with. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)



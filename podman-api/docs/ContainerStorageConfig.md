# ContainerStorageConfig

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**chroot_directories** | Option<**Vec<String>**> | ChrootDirs is an additional set of directories that need to be treated as root directories. Standard bind mounts will be mounted into paths relative to these directories. Optional. | [optional]
**create_working_dir** | Option<**bool**> | Create the working directory if it doesn't exist. If unset, it doesn't create it. Optional. | [optional]
**device_cgroup_rule** | Option<[**Vec<models::LinuxDeviceCgroup>**](LinuxDeviceCgroup.md)> | DeviceCgroupRule are device cgroup rules that allow containers to use additional types of devices. | [optional]
**devices** | Option<[**Vec<models::LinuxDevice>**](LinuxDevice.md)> | Devices are devices that will be added to the container. Optional. | [optional]
**devices_from** | Option<**Vec<String>**> | DevicesFrom specifies that this container will mount the device(s) from other container(s). Optional. | [optional]
**host_device_list** | Option<[**Vec<models::LinuxDevice>**](LinuxDevice.md)> | HostDeviceList is used to recreate the mounted device on inherited containers | [optional]
**image** | Option<**String**> | Image is the image the container will be based on. The image will be used as the container's root filesystem, and its environment vars, volumes, and other configuration will be applied to the container. Conflicts with Rootfs. At least one of Image or Rootfs must be specified. | [optional]
**image_arch** | Option<**String**> | ImageArch is the user-specified image architecture. Used to select a different variant from a manifest list. Optional. | [optional]
**image_os** | Option<**String**> | ImageOS is the user-specified OS of the image. Used to select a different variant from a manifest list. Optional. | [optional]
**image_variant** | Option<**String**> | ImageVariant is the user-specified image variant. Used to select a different variant from a manifest list. Optional. | [optional]
**image_volume_mode** | Option<**String**> | ImageVolumeMode indicates how image volumes will be created. Supported modes are \"ignore\" (do not create), \"tmpfs\" (create as tmpfs), and \"anonymous\" (create as anonymous volumes). The default if unset is anonymous. Optional. | [optional]
**image_volumes** | Option<[**Vec<models::ImageVolume>**](ImageVolume.md)> | Image volumes bind-mount a container-image mount into the container. Optional. | [optional]
**init** | Option<**bool**> | Init specifies that an init binary will be mounted into the container, and will be used as PID1. Optional. | [optional]
**init_path** | Option<**String**> | InitPath specifies the path to the init binary that will be added if Init is specified above. If not specified, the default set in the Libpod config will be used. Ignored if Init above is not set. Optional. | [optional]
**ipcns** | Option<[**models::Namespace**](Namespace.md)> |  | [optional]
**mounts** | Option<[**Vec<models::Mount>**](Mount.md)> | Mounts are mounts that will be added to the container. These will supersede Image Volumes and VolumesFrom volumes where there are conflicts. Optional. | [optional]
**overlay_volumes** | Option<[**Vec<models::OverlayVolume>**](OverlayVolume.md)> | Overlay volumes are named volumes that will be added to the container. Optional. | [optional]
**raw_image_name** | Option<**String**> | RawImageName is the user-specified and unprocessed input referring to a local or a remote image. Optional, but strongly encouraged to be set if Image is set. | [optional]
**rootfs** | Option<**String**> | Rootfs is the path to a directory that will be used as the container's root filesystem. No modification will be made to the directory, it will be directly mounted into the container as root. Conflicts with Image. At least one of Image or Rootfs must be specified. | [optional]
**rootfs_mapping** | Option<**String**> | RootfsMapping specifies if there are UID/GID mappings to apply to the rootfs. Optional. | [optional]
**rootfs_overlay** | Option<**bool**> | RootfsOverlay tells if rootfs is actually an overlay on top of base path. Optional. | [optional]
**rootfs_propagation** | Option<**String**> | RootfsPropagation is the rootfs propagation mode for the container. If not set, the default of rslave will be used. Optional. | [optional]
**secrets** | Option<[**Vec<models::Secret>**](Secret.md)> | Secrets are the secrets that will be added to the container Optional. | [optional]
**shm_size** | Option<**i64**> | ShmSize is the size of the tmpfs to mount in at /dev/shm, in bytes. Conflicts with ShmSize if IpcNS is not private. Optional. | [optional]
**shm_size_systemd** | Option<**i64**> | ShmSizeSystemd is the size of systemd-specific tmpfs mounts specifically /run, /run/lock, /var/log/journal and /tmp. Optional | [optional]
**storage_opts** | Option<**std::collections::HashMap<String, String>**> | StorageOpts is the container's storage options Optional. | [optional]
**volatile** | Option<**bool**> | Volatile specifies whether the container storage can be optimized at the cost of not syncing all the dirty files in memory. Optional. | [optional]
**volumes** | Option<[**Vec<models::NamedVolume>**](NamedVolume.md)> | Volumes are named volumes that will be added to the container. These will supersede Image Volumes and VolumesFrom volumes where there are conflicts. Optional. | [optional]
**volumes_from** | Option<**Vec<String>**> | VolumesFrom is a set of containers whose volumes will be added to this container. The name or ID of the container must be provided, and may optionally be followed by a : and then one or more comma-separated options. Valid options are 'ro', 'rw', and 'z'. Options will be used for all volumes sourced from the container. Optional. | [optional]
**work_dir** | Option<**String**> | WorkDir is the container's working directory. If unset, the default, /, will be used. Optional. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)



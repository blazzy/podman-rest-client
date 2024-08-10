use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Serialize, Deserialize)]
/// ContainerStorageConfig contains information on the storage configuration of a
/// container.
pub struct ContainerStorageConfig {
    /// ChrootDirs is an additional set of directories that need to be
    /// treated as root directories. Standard bind mounts will be mounted
    /// into paths relative to these directories.
    /// Optional.
    pub chroot_directories: Option<Vec<String>>,

    /// Create the working directory if it doesn't exist.
    /// If unset, it doesn't create it.
    /// Optional.
    pub create_working_dir: Option<bool>,

    /// DeviceCgroupRule are device cgroup rules that allow containers
    /// to use additional types of devices.
    pub device_cgroup_rule: Option<Vec<super::super::models::LinuxDeviceCgroup>>,

    /// Devices are devices that will be added to the container.
    /// Optional.
    pub devices: Option<Vec<super::super::models::LinuxDevice>>,

    /// DevicesFrom specifies that this container will mount the device(s) from other container(s).
    /// Optional.
    pub devices_from: Option<Vec<String>>,

    /// HostDeviceList is used to recreate the mounted device on inherited containers
    pub host_device_list: Option<Vec<super::super::models::LinuxDevice>>,

    /// Image is the image the container will be based on. The image will be
    /// used as the container's root filesystem, and its environment vars,
    /// volumes, and other configuration will be applied to the container.
    /// Conflicts with Rootfs.
    /// At least one of Image or Rootfs must be specified.
    pub image: Option<String>,

    /// ImageArch is the user-specified image architecture.
    /// Used to select a different variant from a manifest list.
    /// Optional.
    pub image_arch: Option<String>,

    /// ImageOS is the user-specified OS of the image.
    /// Used to select a different variant from a manifest list.
    /// Optional.
    pub image_os: Option<String>,

    /// ImageVariant is the user-specified image variant.
    /// Used to select a different variant from a manifest list.
    /// Optional.
    pub image_variant: Option<String>,

    /// ImageVolumeMode indicates how image volumes will be created.
    /// Supported modes are "ignore" (do not create), "tmpfs" (create as
    /// tmpfs), and "anonymous" (create as anonymous volumes).
    /// The default if unset is anonymous.
    /// Optional.
    pub image_volume_mode: Option<String>,

    /// Image volumes bind-mount a container-image mount into the container.
    /// Optional.
    pub image_volumes: Option<Vec<super::super::models::ImageVolume>>,

    /// Init specifies that an init binary will be mounted into the
    /// container, and will be used as PID1.
    /// Optional.
    pub init: Option<bool>,

    /// InitPath specifies the path to the init binary that will be added if
    /// Init is specified above. If not specified, the default set in the
    /// Libpod config will be used. Ignored if Init above is not set.
    /// Optional.
    pub init_path: Option<String>,

    pub ipcns: Option<super::super::models::Namespace>,

    /// Mounts are mounts that will be added to the container.
    /// These will supersede Image Volumes and VolumesFrom volumes where
    /// there are conflicts.
    /// Optional.
    pub mounts: Option<Vec<super::super::models::Mount>>,

    /// Overlay volumes are named volumes that will be added to the container.
    /// Optional.
    pub overlay_volumes: Option<Vec<super::super::models::OverlayVolume>>,

    /// RawImageName is the user-specified and unprocessed input referring
    /// to a local or a remote image.
    /// Optional, but strongly encouraged to be set if Image is set.
    pub raw_image_name: Option<String>,

    /// Rootfs is the path to a directory that will be used as the
    /// container's root filesystem. No modification will be made to the
    /// directory, it will be directly mounted into the container as root.
    /// Conflicts with Image.
    /// At least one of Image or Rootfs must be specified.
    pub rootfs: Option<String>,

    /// RootfsMapping specifies if there are UID/GID mappings to apply to the rootfs.
    /// Optional.
    pub rootfs_mapping: Option<String>,

    /// RootfsOverlay tells if rootfs is actually an overlay on top of base path.
    /// Optional.
    pub rootfs_overlay: Option<bool>,

    /// RootfsPropagation is the rootfs propagation mode for the container.
    /// If not set, the default of rslave will be used.
    /// Optional.
    pub rootfs_propagation: Option<String>,

    /// Secrets are the secrets that will be added to the container
    /// Optional.
    pub secrets: Option<Vec<super::super::models::Secret>>,

    /// ShmSize is the size of the tmpfs to mount in at /dev/shm, in bytes.
    /// Conflicts with ShmSize if IpcNS is not private.
    /// Optional.
    pub shm_size: Option<i64>,

    /// ShmSizeSystemd is the size of systemd-specific tmpfs mounts
    /// specifically /run, /run/lock, /var/log/journal and /tmp.
    /// Optional
    pub shm_size_systemd: Option<i64>,

    /// StorageOpts is the container's storage options
    /// Optional.
    pub storage_opts: Option<std::collections::HashMap<String, String>>,

    /// Volatile specifies whether the container storage can be optimized
    /// at the cost of not syncing all the dirty files in memory.
    /// Optional.
    pub volatile: Option<bool>,

    /// Volumes are named volumes that will be added to the container.
    /// These will supersede Image Volumes and VolumesFrom volumes where
    /// there are conflicts.
    /// Optional.
    pub volumes: Option<Vec<super::super::models::NamedVolume>>,

    /// VolumesFrom is a set of containers whose volumes will be added to
    /// this container. The name or ID of the container must be provided, and
    /// may optionally be followed by a : and then one or more
    /// comma-separated options. Valid options are 'ro', 'rw', and 'z'.
    /// Options will be used for all volumes sourced from the container.
    /// Optional.
    pub volumes_from: Option<Vec<String>>,

    /// WorkDir is the container's working directory.
    /// If unset, the default, /, will be used.
    /// Optional.
    pub work_dir: Option<String>,
}

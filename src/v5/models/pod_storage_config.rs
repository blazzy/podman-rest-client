use serde::{Deserialize, Serialize};
#[derive(Default, Debug, Serialize, Deserialize)]
/// PodStorageConfig contains all of the storage related options for the pod and its infra container.
pub struct PodStorageConfig {
    /// Image volumes bind-mount a container-image mount into the pod's infra container.
    /// Optional.
    pub image_volumes: Option<Vec<crate::v5::models::ImageVolume>>,
    /// Mounts are mounts that will be added to the pod.
    /// These will supersede Image Volumes and VolumesFrom volumes where
    /// there are conflicts.
    /// Optional.
    pub mounts: Option<Vec<crate::v5::models::Mount>>,
    /// Overlay volumes are named volumes that will be added to the pod.
    /// Optional.
    pub overlay_volumes: Option<Vec<crate::v5::models::OverlayVolume>>,
    /// ShmSize is the size of the tmpfs to mount in at /dev/shm, in bytes.
    /// Conflicts with ShmSize if IpcNS is not private.
    /// Optional.
    pub shm_size: Option<i64>,
    /// ShmSizeSystemd is the size of systemd-specific tmpfs mounts
    /// specifically /run, /run/lock, /var/log/journal and /tmp.
    /// Optional
    pub shm_size_systemd: Option<i64>,
    /// Volumes are named volumes that will be added to the pod.
    /// These will supersede Image Volumes and VolumesFrom  volumes where
    /// there are conflicts.
    /// Optional.
    pub volumes: Option<Vec<crate::v5::models::NamedVolume>>,
    /// VolumesFrom is a set of containers whose volumes will be added to
    /// this pod. The name or ID of the container must be provided, and
    /// may optionally be followed by a : and then one or more
    /// comma-separated options. Valid options are 'ro', 'rw', and 'z'.
    /// Options will be used for all volumes sourced from the container.
    pub volumes_from: Option<Vec<String>>,
}

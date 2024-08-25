use serde::{Deserialize, Serialize};
#[derive(Default, Debug, Serialize, Deserialize)]
/// Info contains information about the Volume as a whole as provided by
/// the CSI storage plugin.
pub struct Info {
    /// AccessibleTopolgoy is the topology this volume is actually accessible
    /// from.
    #[serde(rename = "AccessibleTopology")]
    pub accessible_topology: Option<Vec<crate::v5::models::Topology>>,
    /// CapacityBytes is the capacity of the volume in bytes. A value of 0
    /// indicates that the capacity is unknown.
    #[serde(rename = "CapacityBytes")]
    pub capacity_bytes: Option<i64>,
    /// VolumeContext is the context originating from the CSI storage plugin
    /// when the Volume is created.
    #[serde(rename = "VolumeContext")]
    pub volume_context: Option<std::collections::HashMap<String, String>>,
    /// VolumeID is the ID of the Volume as seen by the CSI storage plugin. This
    /// is distinct from the Volume's Swarm ID, which is the ID used by all of
    /// the Docker Engine to refer to the Volume. If this field is blank, then
    /// the Volume has not been successfully created yet.
    #[serde(rename = "VolumeID")]
    pub volume_id: Option<String>,
}

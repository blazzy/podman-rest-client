#[derive(Default, Debug)]
pub struct ContainerRestoreLibpod<'a> {
    /// the name of the container when restored from a tar. can only be used with import
    pub name: Option<&'a str>,
    /// keep all temporary checkpoint files
    pub keep: Option<bool>,
    /// checkpoint a container with established TCP connections
    pub tcp_established: Option<bool>,
    /// import the restore from a checkpoint tar.gz
    pub import: Option<bool>,
    /// do not include root file-system changes when exporting. can only be used with import
    pub ignore_root_fs: Option<bool>,
    /// do not restore associated volumes. can only be used with import
    pub ignore_volumes: Option<bool>,
    /// ignore IP address if set statically
    pub ignore_static_ip: Option<bool>,
    /// ignore MAC address if set statically
    pub ignore_static_mac: Option<bool>,
    /// restore a container with file locks
    pub file_locks: Option<bool>,
    /// add restore statistics to the returned RestoreReport
    pub print_stats: Option<bool>,
    /// pod to restore into
    pub pod: Option<&'a str>,
}

#[derive(Default, Debug)]
pub struct ContainerCheckpointLibpod {
    /// keep all temporary checkpoint files
    pub keep: Option<bool>,

    /// leave the container running after writing checkpoint to disk
    pub leave_running: Option<bool>,

    /// checkpoint a container with established TCP connections
    pub tcp_established: Option<bool>,

    /// export the checkpoint image to a tar.gz
    pub export: Option<bool>,

    /// do not include root file-system changes when exporting. can only be used with export
    pub ignore_root_fs: Option<bool>,

    /// do not include associated volumes. can only be used with export
    pub ignore_volumes: Option<bool>,

    /// dump the container's memory information only, leaving the container running. only works on runc 1.0-rc or higher
    pub pre_checkpoint: Option<bool>,

    /// check out the container with previous criu image files in pre-dump. only works on runc 1.0-rc or higher
    pub with_previous: Option<bool>,

    /// checkpoint a container with filelocks
    pub file_locks: Option<bool>,

    /// add checkpoint statistics to the returned CheckpointReport
    pub print_stats: Option<bool>,
}

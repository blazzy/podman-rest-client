#[derive(Default, Debug)]
pub struct PutContainerArchive<'a> {
    /// Path to a directory in the container to extract
    pub path: &'a str,
    /// if unpacking the given content would cause an existing directory to be replaced with a non-directory and vice versa (1 or true)
    pub no_overwrite_dir_non_dir: Option<&'a str>,
    /// copy UID/GID maps to the dest file or di (1 or true)
    pub copy_uidgid: Option<&'a str>,
}

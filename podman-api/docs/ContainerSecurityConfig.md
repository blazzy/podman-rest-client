# ContainerSecurityConfig

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**apparmor_profile** | Option<**String**> | ApparmorProfile is the name of the Apparmor profile the container will use. Optional. | [optional]
**cap_add** | Option<**Vec<String>**> | CapAdd are capabilities which will be added to the container. Conflicts with Privileged. Optional. | [optional]
**cap_drop** | Option<**Vec<String>**> | CapDrop are capabilities which will be removed from the container. Conflicts with Privileged. Optional. | [optional]
**groups** | Option<**Vec<String>**> | Groups are a list of supplemental groups the container's user will be granted access to. Optional. | [optional]
**idmappings** | Option<[**models::IdMappingOptions**](IDMappingOptions.md)> |  | [optional]
**label_nested** | Option<**bool**> | LabelNested indicates whether or not the container is allowed to run fully nested containers including SELinux labelling. Optional. | [optional]
**mask** | Option<**Vec<String>**> | Mask is the path we want to mask in the container. This masks the paths given in addition to the default list. Optional | [optional]
**no_new_privileges** | Option<**bool**> | NoNewPrivileges is whether the container will set the no new privileges flag on create, which disables gaining additional privileges (e.g. via setuid) in the container. Optional. | [optional]
**privileged** | Option<**bool**> | Privileged is whether the container is privileged. Privileged does the following: Adds all devices on the system to the container. Adds all capabilities to the container. Disables Seccomp, SELinux, and Apparmor confinement. (Though SELinux can be manually re-enabled). TODO: this conflicts with things. TODO: this does more. Optional. | [optional]
**procfs_opts** | Option<**Vec<String>**> | ProcOpts are the options used for the proc mount. | [optional]
**read_only_filesystem** | Option<**bool**> | ReadOnlyFilesystem indicates that everything will be mounted as read-only. Optional. | [optional]
**read_write_tmpfs** | Option<**bool**> | ReadWriteTmpfs indicates that when running with a ReadOnlyFilesystem mount temporary file systems. Optional. | [optional]
**seccomp_policy** | Option<**String**> | SeccompPolicy determines which seccomp profile gets applied the container. valid values: empty,default,image | [optional]
**seccomp_profile_path** | Option<**String**> | SeccompProfilePath is the path to a JSON file containing the container's Seccomp profile. If not specified, no Seccomp profile will be used. Optional. | [optional]
**selinux_opts** | Option<**Vec<String>**> | SelinuxProcessLabel is the process label the container will use. If SELinux is enabled and this is not specified, a label will be automatically generated if not specified. Optional. | [optional]
**umask** | Option<**String**> | Umask is the umask the init process of the container will be run with. | [optional]
**unmask** | Option<**Vec<String>**> | Unmask a path in the container. Some paths are masked by default, preventing them from being accessed within the container; this undoes that masking. If ALL is passed, all paths will be unmasked. Optional. | [optional]
**user** | Option<**String**> | User is the user the container will be run as. Can be given as a UID or a username; if a username, it will be resolved within the container, using the container's /etc/passwd. If unset, the container will be run as root. Optional. | [optional]
**userns** | Option<[**models::Namespace**](Namespace.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)



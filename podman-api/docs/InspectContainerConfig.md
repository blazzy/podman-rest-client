# InspectContainerConfig

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**annotations** | Option<**std::collections::HashMap<String, String>**> | Container annotations | [optional]
**attach_stderr** | Option<**bool**> | Unused, at present | [optional]
**attach_stdin** | Option<**bool**> | Unused, at present | [optional]
**attach_stdout** | Option<**bool**> | Unused, at present | [optional]
**chroot_dirs** | Option<**Vec<String>**> | ChrootDirs is an additional set of directories that need to be treated as root directories. Standard bind mounts will be mounted into paths relative to these directories. | [optional]
**cmd** | Option<**Vec<String>**> | Container command | [optional]
**create_command** | Option<**Vec<String>**> | CreateCommand is the full command plus arguments of the process the container has been created with. | [optional]
**domainname** | Option<**String**> | Container domain name - unused at present | [optional]
**entrypoint** | Option<**Vec<String>**> | Container entrypoint | [optional]
**env** | Option<**Vec<String>**> | Container environment variables | [optional]
**healthcheck** | Option<[**models::Schema2HealthConfig**](Schema2HealthConfig.md)> |  | [optional]
**healthcheck_on_failure_action** | Option<**String**> | HealthcheckOnFailureAction defines an action to take once the container turns unhealthy. | [optional]
**hostname** | Option<**String**> | Container hostname | [optional]
**image** | Option<**String**> | Container image | [optional]
**labels** | Option<**std::collections::HashMap<String, String>**> | Container labels | [optional]
**on_build** | Option<**String**> | On-build arguments - presently unused. More of Buildah's domain. | [optional]
**open_stdin** | Option<**bool**> | Whether the container leaves STDIN open | [optional]
**passwd** | Option<**bool**> | Passwd determines whether or not podman can add entries to /etc/passwd and /etc/group | [optional]
**secrets** | Option<[**Vec<models::InspectSecret>**](InspectSecret.md)> | Secrets are the secrets mounted in the container | [optional]
**stdin_once** | Option<**bool**> | Whether STDIN is only left open once. Presently not supported by Podman, unused. | [optional]
**stop_signal** | Option<**String**> | Container stop signal | [optional]
**stop_timeout** | Option<**i32**> | StopTimeout is time before container is stopped when calling stop | [optional]
**systemd_mode** | Option<**bool**> | SystemdMode is whether the container is running in systemd mode. In systemd mode, the container configuration is customized to optimize running systemd in the container. | [optional]
**timeout** | Option<**i32**> | Timeout is time before container is killed by conmon | [optional]
**timezone** | Option<**String**> | Timezone is the timezone inside the container. Local means it has the same timezone as the host machine | [optional]
**tty** | Option<**bool**> | Whether the container creates a TTY | [optional]
**umask** | Option<**String**> | Umask is the umask inside the container. | [optional]
**user** | Option<**String**> | User the container was launched with | [optional]
**volumes** | Option<[**std::collections::HashMap<String, serde_json::Value>**](serde_json::Value.md)> | Unused, at present. I've never seen this field populated. | [optional]
**working_dir** | Option<**String**> | Container working directory | [optional]
**sd_notify_mode** | Option<**String**> | SdNotifyMode is the sd-notify mode of the container. | [optional]
**sd_notify_socket** | Option<**String**> | SdNotifySocket is the NOTIFY_SOCKET in use by/configured for the container. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)



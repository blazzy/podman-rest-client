# InspectExecSession

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**can_remove** | Option<**bool**> | CanRemove is legacy and used purely for compatibility reasons. Will always be set to true, unless the exec session is running. | [optional]
**container_id** | Option<**String**> | ContainerID is the ID of the container this exec session is attached to. | [optional]
**detach_keys** | Option<**String**> | DetachKeys are the detach keys used by the exec session. If set to \"\" the default keys are being used. Will show \"<none>\" if no detach keys are set. | [optional]
**exit_code** | Option<**i64**> | ExitCode is the exit code of the exec session. Will be set to 0 if the exec session has not yet exited. | [optional]
**id** | Option<**String**> | ID is the ID of the exec session. | [optional]
**open_stderr** | Option<**bool**> | OpenStderr is whether the container's STDERR stream will be attached. Always set to true if the exec session created a TTY. | [optional]
**open_stdin** | Option<**bool**> | OpenStdin is whether the container's STDIN stream will be attached to. | [optional]
**open_stdout** | Option<**bool**> | OpenStdout is whether the container's STDOUT stream will be attached. Always set to true if the exec session created a TTY. | [optional]
**pid** | Option<**i64**> | Pid is the PID of the exec session's process. Will be set to 0 if the exec session is not running. | [optional]
**process_config** | Option<[**models::InspectExecProcess**](InspectExecProcess.md)> |  | [optional]
**running** | Option<**bool**> | Running is whether the exec session is running. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)



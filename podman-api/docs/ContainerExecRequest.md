# ContainerExecRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**attach_stderr** | Option<**bool**> | Attach to stderr of the exec command | [optional]
**attach_stdin** | Option<**bool**> | Attach to stdin of the exec command | [optional]
**attach_stdout** | Option<**bool**> | Attach to stdout of the exec command | [optional]
**cmd** | Option<**Vec<String>**> | Command to run, as a string or array of strings. | [optional]
**detach_keys** | Option<**String**> | \"Override the key sequence for detaching a container. Format is a single character [a-Z] or ctrl-<value> where <value> is one of: a-z, @, ^, [, , or _.\"  | [optional]
**env** | Option<**Vec<String>**> | A list of environment variables in the form [\"VAR=value\", ...] | [optional]
**privileged** | Option<**bool**> | Runs the exec process with extended privileges | [optional][default to false]
**tty** | Option<**bool**> | Allocate a pseudo-TTY | [optional]
**user** | Option<**String**> | \"The user, and optionally, group to run the exec process inside the container. Format is one of: user, user:group, uid, or uid:gid.\"  | [optional]
**working_dir** | Option<**String**> | The working directory for the exec process inside the container. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)



# ImageConfig

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**args_escaped** | Option<**bool**> | ArgsEscaped  Deprecated: This field is present only for legacy compatibility with Docker and should not be used by new image builders.  It is used by Docker for Windows images to indicate that the `Entrypoint` or `Cmd` or both, contains only a single element array, that is a pre-escaped, and combined into a single string `CommandLine`. If `true` the value in `Entrypoint` or `Cmd` should be used as-is to avoid double escaping. https://github.com/opencontainers/image-spec/pull/892 | [optional]
**cmd** | Option<**Vec<String>**> | Cmd defines the default arguments to the entrypoint of the container. | [optional]
**entrypoint** | Option<**Vec<String>**> | Entrypoint defines a list of arguments to use as the command to execute when the container starts. | [optional]
**env** | Option<**Vec<String>**> | Env is a list of environment variables to be used in a container. | [optional]
**exposed_ports** | Option<[**std::collections::HashMap<String, serde_json::Value>**](serde_json::Value.md)> | ExposedPorts a set of ports to expose from a container running this image. | [optional]
**labels** | Option<**std::collections::HashMap<String, String>**> | Labels contains arbitrary metadata for the container. | [optional]
**stop_signal** | Option<**String**> | StopSignal contains the system call signal that will be sent to the container to exit. | [optional]
**user** | Option<**String**> | User defines the username or UID which the process in the container should run as. | [optional]
**volumes** | Option<[**std::collections::HashMap<String, serde_json::Value>**](serde_json::Value.md)> | Volumes is a set of directories describing where the process is likely write data specific to a container instance. | [optional]
**working_dir** | Option<**String**> | WorkingDir sets the current working directory of the entrypoint process in the container. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)



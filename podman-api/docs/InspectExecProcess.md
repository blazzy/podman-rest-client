# InspectExecProcess

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**arguments** | Option<**Vec<String>**> | Arguments are the arguments to the entrypoint command of the exec session. | [optional]
**entrypoint** | Option<**String**> | Entrypoint is the entrypoint for the exec session (the command that will be executed in the container). | [optional]
**privileged** | Option<**bool**> | Privileged is whether the exec session will be started with elevated privileges. | [optional]
**tty** | Option<**bool**> | Tty is whether the exec session created a terminal. | [optional]
**user** | Option<**String**> | User is the user the exec session was started as. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)



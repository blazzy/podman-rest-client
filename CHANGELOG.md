# Changelog

## Unreleased

* Improve formatting in generated docs with extra whitespace and removal redundant lines

## v0.12.1 - 2024-08-21

* ImageExportLibpod and ContainerExportLibpod operations return streams of bytes ([#14](https://github.com/blazzy/podman-rest-client/pull/14))

## v0.12.0

### Breaking Changes

* The old enum `Error` is now `ClientError`
* The struct `APIClient` and the module `api_common` are no longer available
* The new enum `Error` is represents the error returned from api requests

## v0.11.1

* Fix docs.rs build

## v0.11.0

* Use new api client generator to generate client from swagger file. Big breaking changes
* New feature flag for ssh support
* New feature flag for early v4 support

### Breaking Changes

* Query and Header parameters are now provided through structs from the `params` module
* Body parameters are no longer optional.
* Some i32/u32 fields became i16/u16
* API functions no longer have the _api suffix
* API functions not in scope unless the `v5::Client` trait is in scope or
alternatively they can be invoked via `client.v5()`

## v0.10.2

* Fix issue parsing error on pod deletion ([#14](https://github.com/blazzy/podman-rest-client/pull/14))

## v0.10.1

* Fix issue creating containers with mounted volumes ([#12](https://github.com/blazzy/podman-rest-client/pull/12))

## v0.10.0

* Parse error bodies whe encountering API errors ([#11](https://github.com/blazzy/podman-rest-client/pull/11))

## v0.9.1

* Fix for Config::guess on Linux ([#7](https://github.com/blazzy/podman-rest-client/pull/7))

## v0.9.0

* Config guessing logic on linux will return an error if a socket is not found
* Config guessing logic will also try to use the system socket if there is no
user socket
* README.md documents some notes on initializing the podman socket

### Breaking Changes

* `guess_configuration` is now `Config::guess`

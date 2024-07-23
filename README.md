# podman-rest-client

[![Crates.io Version](https://img.shields.io/crates/v/podman-rest-client)](https://crates.io/crates/podman-rest-client)
[![docs.rs](https://docs.rs/podman-rest-client/badge.svg)](https://docs.rs/podman-rest-client)
[![MIT licensed](https://img.shields.io/badge/license-MIT-blue.svg)](./LICENSE)
[![Build](https://img.shields.io/github/actions/workflow/status/blazzy/podman-rest-client/main.yml?branch=main)](https://github.com/blazzy/podman-rest-client/actions)

<!-- cargo-rdme start -->

Provides an interface for querying the Podman REST API. Most of the interface is generated from
the the official Podman swagger file. This crate adds a layer to make it possible to connect to
the podman rest api over ssh to a unix socket and directly to a unix socket. Connections over
ssh are  commonly necessary on macOs where the container runtime runs in a virtual machine
accessible over ssh.


## API Compatibility

This crate currently only works with version 5 of the podman API. There are suffucient
differences between version 3, 4, and 5 that a lot of calls will not work in an older version.
`podman --version` will reveal what version you are using.

## Podman Socket

Note that podman does not run in a client/server model like docker does so there usually isn't
a socket you can connect to by default. You might need to enable the socket for the library to
connect to. For example on linux you might need to run something like this:

```sh
systemctl --user enable --now podman.socket
```

## Usage

### Linux

On linux you might initialize a client like this

```rust
use podman_rest_client::PodmanRestClient;
use podman_rest_client::Config;

// Initialize a client
let client = PodmanRestClient::new(Config {
    uri: "unix:///run/user/501/podman/podman.sock".to_string(),
    identity_file: None,
}).await.unwrap();

// Fetch a list of container images
let images = client.images_api().image_list_libpod(None,None).await.unwrap();
```
### MacOs

On macOs you might initialize a client like this with an ssh url and identity file

```rust
let client = PodmanRestClient::new(Config {
    uri: "ssh://core@127.0.0.1:63169/run/user/501/podman/podman.sock".to_string(),
    identity_file: Some("/path/to/identity_file".into()),
}).await.unwrap();
```

### Config::guess

You can also use `Config::guess()` which tries to find the default path to the podman
socket depending on the platform you are on.

```rust
// Setup the default configuration
let config = Config::guess().await.unwrap();

// Initialize a client
let client = PodmanRestClient::new(config).await.unwrap();
```

<!-- cargo-rdme end -->

## Changelog

### v0.10.1

* Fix issue creating containers with mounted volumes [#12](https://github.com/blazzy/podman-rest-client/pull/12)

### v0.10.0

* Parse error bodies whe encountering API errors [#11](https://github.com/blazzy/podman-rest-client/pull/11)

### v0.9.1

* Fix for Config::guess on Linux [#7](https://github.com/blazzy/podman-rest-client/pull/7)

### v0.9.0

* Config guessing logic on linux will return an error if a socket is not found
* Config guessing logic will also try to use the system socket if there is no
user socket
* README.md documents some notes on initializing the podman socket

#### Breaking Changes

* `guess_configuration` is now `Config::guess`

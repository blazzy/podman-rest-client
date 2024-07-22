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


### API Compatibility

This crate currently only works with version 5 of the podman API. There are suffucient
differences between version 3, 4, and 5 that a lot of calls will not work in an older version.
`podman --version` will reveal what version you are using.

### Podman Socket

Note that podman does not run in a client/server model like docker does so there usually isn't
a socket you can connect to by default. You would need to enable the socket for the library to
connect to. For example on linux you might need to run something like this:

```sh
systemctl --user enable --now podman.socket
```

### Usage

```rust
use podman_rest_client::PodmanRestClient;
use podman_rest_client::Config;

let ssh_client = PodmanRestClient::new(Config {
    uri: "ssh://core@127.0.0.1:63169/run/user/501/podman/podman.sock".to_string(),
    identity_file: Some("/path/to/identity_file".into()),
}).await.unwrap();

let unix_client = PodmanRestClient::new(Config {
    uri: "unix:///run/user/501/podman/podman.sock".to_string(),
    identity_file: None,
}).await.unwrap();
```

You can also use `Config::guess()` which tries to find the default path to the podman
socket depending on the platform you are on.

```rust
use podman_rest_client::PodmanRestClient;
use podman_rest_client::Config;

// Setup the default configuration
let config = Config::guess().await.unwrap();

// Initialize a client
let client = PodmanRestClient::new(config).await.unwrap();

// Fetch a list of container images
let images = client.images_api().image_list_libpod(None,None).await.unwrap();
```

<!-- cargo-rdme end -->

## Changelog

### v0.9.0

* Config guessing logic on linux will return an error if a socket is not found
* Config guessing logic will also try to use the system socket if there is no
user socket
* README.md documents some notes on initializing the podman socket

#### Breaking Changes

* `guess_configuration` is now `Config::guess`

# podman-rest-client

[![Crates.io Version](https://img.shields.io/crates/v/podman-rest-client)](https://crates.io/crates/podman-rest-client)
[![docs.rs](https://docs.rs/podman-rest-client/badge.svg)](https://docs.rs/podman-rest-client)
[![MIT licensed](https://img.shields.io/badge/license-MIT-blue.svg)](./LICENSE)
[![Build](https://img.shields.io/github/actions/workflow/status/blazzy/podman-rest-client/main.yml?branch=main)](https://github.com/blazzy/podman-rest-client/actions)


<!-- cargo-rdme start -->

Provides an interface for querying the podman rest api. Most of the interface is generated from
the the official podman swagger file. This crate adds a layer to make it possible to connect to
the podman rest api over ssh to a unix socket and directl to a unix socket. Connections over
ssh are  commonly necessary on macOs where the container runtime runs in a virtual machine
accessible over ssh.

### Usage

```rust
use podman_rest_client::PodmanRestClient;
use podman_rest_client::guess_configuration;

// Setup the default configuration
let config = guess_configuration().await.unwrap();

// Initialize a client
let client = PodmanRestClient::new(config).await.unwrap();

// Fetch a list of container images
let images = client.images_api().image_list_libpod(None,None).await.unwrap();
```

`guess_configuration` tries to find the default path to the podman socket depending on the
platform you are on. You can also manually create clients configurations:

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

<!-- cargo-rdme end -->

## TODO

* Make ssh support a crate feature that can be excluded.
* Attempt to re-establish the ssh connection if lost. (Alternatively would it
be so horrible to just establish a new ssh connection on each request?)

# podman-rest-client
<!-- cargo-rdme start -->

Provides an interface for querying the podman rest api. Most of the interface is generated from
the the official podman swagger file. This crate adds a layer to make it possible to connect to
the podman rest api over ssh to a unix socket and directl to a unix socket. Connections over
ssh are  commonly necessary on macOs where the container runtime runs in a virtual machine
accessible over ssh.

### Connecting via a unix socket on Linux

```rust
use podman_rest_client::PodmanRestClient;

let uri = "unix://path_to_unix_socket";

let client = PodmanRestClient::new(uri, None).await.unwrap();

let images = client.images_api().image_list_libpod(None,None).await.unwrap();
```

### Connecting via ssh from macOS

```rust
use podman_rest_client::PodmanRestClient;

let uri = "ssh://core@127.0.0.1:63169/run/user/501/podman/podman.sock";
let key: Option<String> = Some("/path/to/key".into());

let client = PodmanRestClient::new(uri, key).await.unwrap();

let images = client.images_api().image_list_libpod(None,None).await.unwrap();
```

<!-- cargo-rdme end -->

## TODO

* Make ssh support a crate feature that can be excluded.
* Attempt to re-establish the ssh connection if lost. (Alternatively would it
be so horrible to just establish a new ssh connection on each request?)

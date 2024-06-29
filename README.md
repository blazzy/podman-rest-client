# podman-rest-client
<!-- cargo-rdme start -->

Provides an interface for querying the podman rest api. Most of the interface is generated from
the the official podman swagger file. This crate adds a layer to make it possible to connect to
podman over ssh. This is commonly necessary on macOs where the container runtime runs in a
virtual machine you connect to over ssh.

```rust
let client = PodmanRestCli
```

<!-- cargo-rdme end -->

## TODO

* Make ssh support a crate feature that can be excluded.
* Attempt to re-establish the ssh connection if lost. (Alternatively would it
be so horrible to just establish a new ssh connection on each request?)

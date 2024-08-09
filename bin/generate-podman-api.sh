# Use openapi-client-gen to auto generate the basic podman-autogen-api client
cargo run -p openapi-client-gen ./swagger/swagger-v5.1.0.modified.yaml ./podman-autogen-api

# Format the auto generated code
cargo fmt

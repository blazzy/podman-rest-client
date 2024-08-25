# Use openapi-client-gen to auto generate the basic podman-autogen-api client
cargo run -p openapi-client-gen -- --skip-default-client --module --common-dir ./src/api_common --hash-maps-always-nullable ./swagger/swagger-v5.1.0.modified.yaml ./src/v5

cargo run -p openapi-client-gen -- --skip-default-client --module --common-dir ./src/api_common --hash-maps-always-nullable ./swagger/swagger-v4.9.modified.yaml ./src/v4

# Format the auto generated code
cargo fmt

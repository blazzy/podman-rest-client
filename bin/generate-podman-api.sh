# Use openapi-generator-cli to auto generate a basic podman-api client
java -jar ../openapi-generator/modules/openapi-generator-cli/target/openapi-generator-cli.jar generate -i https://storage.googleapis.com/libpod-master-releases/swagger-v5.1.0.yaml -g rust --skip-validate-spec -o podman-autogen-api  --package-name podman-autogen-api --library hyper 

# Format the auto generated code
cargo fmt

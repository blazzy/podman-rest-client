fn main() {
    let v4_enabled = std::env::var("CARGO_FEATURE_V4").is_ok();
    let v5_enabled = std::env::var("CARGO_FEATURE_V5").is_ok();

    if v4_enabled && v5_enabled {
        let error =
            "Features `v5` and `v4` cannot be enabled at the same time for podman_rest_client";
        println!("cargo:error={}", error);
        panic!("{}", error);
    }
}

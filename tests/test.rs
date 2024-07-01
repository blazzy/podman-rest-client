use podman_rest_client::{guess_configuration, Config, PodmanRestClient};
use podman_rest_client::Error;
use assert_matches::assert_matches;

#[tokio::test]
async fn it_connects_to_a_unix_socket() {
    let client = PodmanRestClient::new(Config {
        uri: "unix:///tmp/sock".to_string(),
        identity_file: None,
    })
    .await;
    assert!(client.is_ok());
}

#[tokio::test]
async fn it_can_list_images() {
    let config = guess_configuration().await.unwrap();
    let client = PodmanRestClient::new(config).await.unwrap();
    let images = client.images_api().image_list_libpod(None, None).await;
    assert!(images.is_ok());
}

#[tokio::test]
async fn it_errors_on_invalid_uris() {
    let err = PodmanRestClient::new(Config {
        uri: "tcp:///127.0.0.1:80".to_string(),
        identity_file: None,
    }).await.err().unwrap();

    assert_matches!(err, Error::InvalidScheme);
}

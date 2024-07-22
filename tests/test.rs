use assert_matches::assert_matches;
use podman_rest_client::Error;
use podman_rest_client::{guess_configuration, Config, PodmanRestClient};

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
    let result = client.images_api().image_list_libpod(None, None).await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn it_can_pull_images() {
    let config = guess_configuration().await.unwrap();
    let client = PodmanRestClient::new(config).await.unwrap();
    let pull_report = client
        .images_api()
        .image_pull_libpod(
            Some("docker.io/library/nginx:latest"),
            Some(true),
            Some(false),
            None,
            None,
            None,
            None,
            None,
            None,
            None,
        )
        .await
        .unwrap();
    assert!(pull_report.error.is_none());
    assert!(pull_report.id.is_some());
}

#[tokio::test]
async fn it_can_create_a_container() {
    let config = guess_configuration().await.unwrap();
    let client = PodmanRestClient::new(config).await.unwrap();
    let create = podman_rest_client::models::SpecGenerator {
        name: Some("podman_rest_client_test".into()),
        image: Some("docker.io/library/nginx:latest".into()),
        ..podman_rest_client::models::SpecGenerator::default()
    };
    let response = client
        .containers_api()
        .container_create_libpod(create)
        .await;
    assert!(response.is_ok());

    // Clean up container. This cleanup is probably fragile and might need someone to manually delete
    // containers to fix this test
    let response = client
        .containers_api()
        .container_delete_libpod("podman_rest_client_test", None, None, None, None, None)
        .await;
    assert!(response.is_ok());
}

#[tokio::test]
async fn it_errors_on_invalid_uris() {
    let err = PodmanRestClient::new(Config {
        uri: "tcp:///127.0.0.1:80".to_string(),
        identity_file: None,
    })
    .await
    .err()
    .unwrap();

    assert_matches!(err, Error::InvalidScheme);
}

use assert_matches::assert_matches;
use podman_rest_client::models;
use podman_rest_client::Error;
use podman_rest_client::{Config, PodmanRestClient};

mod common;

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
    let config = Config::guess().await.unwrap();
    let client = PodmanRestClient::new(config).await.unwrap();
    let result = client.images_api().image_list_libpod(None, None).await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn it_can_create_a_volume() {
    let config = Config::guess().await.unwrap();
    let client = PodmanRestClient::new(config).await.unwrap();

    let create = models::VolumeCreateOptions {
        name: Some("podman_rest_client_volume_test".into()),
        ..models::VolumeCreateOptions::default()
    };
    let result = client
        .volumes_api()
        .volume_create_libpod(Some(create))
        .await;
    result.expect("Failed to create a volume");

    common::delete_volume(&client, "podman_rest_client_volume_test").await;
}

#[tokio::test]
async fn it_can_run_in_a_thread() {
    let config = Config::guess().await.unwrap();
    let client = PodmanRestClient::new(config).await.unwrap();

    let handle = tokio::spawn(async move {
        let result = client.images_api().image_list_libpod(None, None).await;
        assert!(result.is_ok());
    });

    assert!(handle.await.is_ok());
}

#[tokio::test]
async fn it_can_pull_images() {
    let config = Config::guess().await.unwrap();
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
    let config = Config::guess().await.unwrap();
    let client = PodmanRestClient::new(config).await.unwrap();

    common::pull_nginx_image(&client).await;

    let create = models::SpecGenerator {
        name: Some("podman_rest_client_creation_test".into()),
        image: Some("docker.io/library/nginx:latest".into()),
        ..models::SpecGenerator::default()
    };

    client
        .containers_api()
        .container_create_libpod(create)
        .await
        .expect("Failed to create pod");

    common::delete_container(&client, "podman_rest_client_creation_test").await;
}

#[ignore = "Spec seems to broken. Possible related issue: https://github.com/containers/podman/issues/13092"]
#[tokio::test]
async fn it_can_create_a_container_with_a_volume_mounted() {
    let config = Config::guess().await.unwrap();
    let client = PodmanRestClient::new(config).await.unwrap();

    common::pull_nginx_image(&client).await;
    common::create_volume(&client, "podman_rest_client_container_volume_test").await;

    let create = models::SpecGenerator {
        name: Some("podman_rest_client_container_volume_test".into()),
        image: Some("docker.io/library/nginx:latest".into()),
        mounts: Some(vec![models::Mount {
            source: Some("podman_rest_client_container_volume_test".into()),
            target: Some("/foo".into()),
            ..models::Mount::default()
        }]),
        ..models::SpecGenerator::default()
    };

    client
        .containers_api()
        .container_create_libpod(create)
        .await
        .expect("Failed to create pod");

    common::delete_volume(&client, "podman_rest_client_container_volume_test").await;
}

#[ignore = "Breaking on ci server because its running an older podman version need to upgrade"]
#[tokio::test]
async fn it_can_inspect_a_container() {
    let config = Config::guess().await.unwrap();
    let client = PodmanRestClient::new(config).await.unwrap();

    common::pull_nginx_image(&client).await;
    common::create_nginx_container(&client, "podman_rest_client_inspect_test").await;

    client
        .containers_api()
        .container_inspect_libpod("podman_rest_client_inspect_test", None)
        .await
        .expect("Failed to inspect pod");

    common::delete_container(&client, "podman_rest_client_inspect_test").await;
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

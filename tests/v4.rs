#![cfg(feature = "v4")]

use assert_matches::assert_matches;
use podman_rest_client::v4::models;
use podman_rest_client::v4::params;
use podman_rest_client::v4::Client;
use podman_rest_client::Error;
use podman_rest_client::{Config, PodmanRestClient};

mod v4_common;
use v4_common as common;

#[tokio::test]
async fn it_connects_to_a_unix_socket() {
    common::test_init().await;

    let client = PodmanRestClient::new(Config {
        uri: "unix:///tmp/sock".to_string(),
        identity_file: None,
    })
    .await;
    assert!(client.is_ok());
}

#[tokio::test]
async fn it_can_list_images() {
    common::test_init().await;

    let config = Config::guess().await.unwrap();
    let client = PodmanRestClient::new(config).await.unwrap();
    let result = client.images().image_list_libpod(None).await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn it_can_create_a_volume() {
    common::test_init().await;

    let config = Config::guess().await.unwrap();
    let client = PodmanRestClient::new(config).await.unwrap();

    let create = models::VolumeCreateOptions {
        name: Some("podman_rest_client_volume_test".into()),
        ..models::VolumeCreateOptions::default()
    };
    let result = client.volumes().volume_create_libpod(create).await;
    result.expect("Failed to create a volume");

    common::delete_volume(&client, "podman_rest_client_volume_test").await;
}

#[tokio::test]
async fn it_can_run_in_a_thread() {
    common::test_init().await;

    let config = Config::guess().await.unwrap();
    let client = PodmanRestClient::new(config).await.unwrap();

    let handle = tokio::spawn(async move {
        let result = client.images().image_list_libpod(None).await;
        assert!(result.is_ok());
    });

    assert!(handle.await.is_ok());
}

#[tokio::test]
async fn it_can_pull_images() {
    common::test_init().await;

    let config = Config::guess().await.unwrap();
    let client = PodmanRestClient::new(config).await.unwrap();
    let pull_report = client
        .images()
        .image_pull_libpod(Some(params::ImagePullLibpod {
            reference: Some("docker.io/library/nginx:latest"),
            ..Default::default()
        }))
        .await
        .unwrap();
    assert!(pull_report.error.is_none());
    assert!(pull_report.id.is_some());
}

#[tokio::test]
async fn it_can_create_a_container() {
    common::test_init().await;

    let config = Config::guess().await.unwrap();
    let client = PodmanRestClient::new(config).await.unwrap();

    common::pull_nginx_image(&client).await;

    let create = models::SpecGenerator {
        name: Some("podman_rest_client_creation_test".into()),
        image: Some("docker.io/library/nginx:latest".into()),
        ..models::SpecGenerator::default()
    };

    client
        .containers()
        .container_create_libpod(create)
        .await
        .expect("Failed to create container");

    common::delete_container(&client, "podman_rest_client_creation_test").await;
}

#[tokio::test]
async fn it_can_create_a_container_with_ports() {
    common::test_init().await;

    let config = Config::guess().await.unwrap();
    let client = PodmanRestClient::new(config).await.unwrap();

    common::pull_nginx_image(&client).await;

    let name = "podman_rest_client_creation_with_ports_test";

    let create = models::SpecGenerator {
        name: Some(name.into()),
        image: Some("docker.io/library/nginx:latest".into()),
        portmappings: Some(vec![models::PortMapping {
            container_port: Some(80),
            host_port: Some(8000),
            range: Some(10),
            ..models::PortMapping::default()
        }]),
        ..models::SpecGenerator::default()
    };

    client
        .containers()
        .container_create_libpod(create)
        .await
        .expect("Failed to create container");

    let mut list = client
        .containers()
        .container_list_libpod(Some(params::ContainerListLibpod {
            all: Some(true),
            filters: Some(&format!(r#"{{"name": ["{}"]}}"#, name)),
            ..Default::default()
        }))
        .await
        .expect("Failed to list containers");

    let mapping = &list.pop().unwrap().ports.unwrap()[0];
    assert_eq!(mapping.range, Some(10_u16));

    common::delete_container(&client, name).await;
}

#[tokio::test]
async fn it_can_create_a_container_with_a_volume_mounted() {
    common::test_init().await;

    let config = Config::guess().await.unwrap();
    let client = PodmanRestClient::new(config).await.unwrap();

    common::pull_nginx_image(&client).await;
    common::create_volume(&client, "podman_rest_client_container_volume_test").await;

    let create = models::SpecGenerator {
        name: Some("podman_rest_client_container_volume_test".into()),
        image: Some("docker.io/library/nginx:latest".into()),
        mounts: Some(vec![models::Mount {
            source: Some("podman_rest_client_container_volume_test".into()),
            destination: Some("/my-volume".into()),
            ..models::Mount::default()
        }]),
        ..models::SpecGenerator::default()
    };

    client
        .containers()
        .container_create_libpod(create)
        .await
        .expect("Failed to create container");

    common::delete_container(&client, "podman_rest_client_container_volume_test").await;
    common::delete_volume(&client, "podman_rest_client_container_volume_test").await;
}

#[tokio::test]
async fn it_can_inspect_a_container() {
    common::test_init().await;

    let config = Config::guess().await.unwrap();
    let client = PodmanRestClient::new(config).await.unwrap();

    common::pull_nginx_image(&client).await;
    common::create_nginx_container(&client, "podman_rest_client_inspect_test").await;

    client
        .containers()
        .container_inspect_libpod("podman_rest_client_inspect_test", None)
        .await
        .expect("Failed to inspect container");

    common::delete_container(&client, "podman_rest_client_inspect_test").await;
}

#[tokio::test]
async fn it_can_list_containers() {
    common::test_init().await;

    let config = Config::guess().await.unwrap();
    let client = PodmanRestClient::new(config).await.unwrap();

    common::pull_nginx_image(&client).await;
    common::create_nginx_container(&client, "podman_rest_client_list_contaienr_test").await;

    client
        .containers()
        .container_list_libpod(Some(params::ContainerListLibpod {
            all: Some(true),
            ..Default::default()
        }))
        .await
        .expect("Failed to list containers");

    common::delete_container(&client, "podman_rest_client_list_contaienr_test").await;
}

#[tokio::test]
async fn it_can_create_a_pod() {
    common::test_init().await;

    let config = Config::guess().await.unwrap();
    let client = PodmanRestClient::new(config).await.unwrap();

    let create = models::PodSpecGenerator {
        name: Some("podman_rest_client_create_pod_test".into()),
        ..models::PodSpecGenerator::default()
    };

    client
        .pods()
        .pod_create_libpod(create)
        .await
        .expect("Failed to create pod");

    common::delete_pod(&client, "podman_rest_client_create_pod_test").await;
}

#[tokio::test]
async fn it_errors_on_invalid_uris() {
    common::test_init().await;

    let err = PodmanRestClient::new(Config {
        uri: "tcp:///127.0.0.1:80".to_string(),
        identity_file: None,
    })
    .await
    .err()
    .unwrap();

    assert_matches!(err, Error::InvalidScheme);
}

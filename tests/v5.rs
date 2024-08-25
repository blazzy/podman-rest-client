#![cfg(feature = "v5")]

use assert_matches::assert_matches;

use bytes::Bytes;
use futures::StreamExt;
use futures::TryStreamExt;
use podman_rest_client::v5::apis::*;
use podman_rest_client::v5::models;
use podman_rest_client::v5::params;
use podman_rest_client::v5::Client;
use podman_rest_client::AttachFrame;
use podman_rest_client::AttachFrameStream;
use podman_rest_client::ClientError;
use podman_rest_client::{Config, PodmanRestClient};
use tokio::io::AsyncReadExt;
use tokio::io::AsyncWriteExt;

mod v5_common;
use v5_common as common;

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
    let result = client.volume_create_libpod(create).await;
    result.expect("Failed to create a volume");

    common::delete_volume(&client, "podman_rest_client_volume_test").await;
}

#[tokio::test]
async fn it_can_run_call_async_methods_in_threads() {
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
async fn it_can_call_stream_functions_in_a_thread() {
    common::test_init().await;

    let config = Config::guess().await.unwrap();
    let client = PodmanRestClient::new(config).await.unwrap();

    common::pull_nginx_image(&client).await;
    common::create_nginx_container(&client, "podman_rest_client_stream_thread_test").await;

    let config = Config::guess().await.unwrap();
    let client = PodmanRestClient::new(config).await.unwrap();

    let handle = tokio::spawn(async move {
        let stream = client
            .containers()
            .container_export_libpod("podman_rest_client_stream_thread_test");
        let bytes: Vec<u8> = stream.map_ok(|b| b.to_vec()).try_concat().await.unwrap();
        let c = std::io::Cursor::new(bytes);

        assert!(tar::Archive::new(c).entries().unwrap().count() > 0);

        common::delete_container(&client, "podman_rest_client_stream_thread_test").await;
    });

    assert!(handle.await.is_ok());
}

#[tokio::test]
async fn it_can_pull_images() {
    common::test_init().await;

    let config = Config::guess().await.unwrap();
    let client = PodmanRestClient::new(config).await.unwrap();
    let pull_report = client
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

    assert_matches!(err, ClientError::InvalidScheme);
}

#[tokio::test]
async fn it_exports_images() {
    common::test_init().await;

    let config = Config::guess().await.unwrap();
    let client = PodmanRestClient::new(config).await.unwrap();

    futures::join!(
        common::pull_alpine_image(&client),
        common::pull_nginx_image(&client),
    );

    let params = params::ImageExportLibpod {
        compress: None,
        format: Some("docker-archive"),
        oci_accept_uncompressed_layers: None,
        references: Some(["nginx", "alpine"].to_vec()),
    };
    let stream = client.images().image_export_libpod(Some(params));

    let bytes: Vec<u8> = stream.map_ok(|b| b.to_vec()).try_concat().await.unwrap();
    let c = std::io::Cursor::new(bytes);

    assert!(tar::Archive::new(c).entries().unwrap().count() > 0)
}

#[tokio::test]
async fn it_exports_containers() {
    common::test_init().await;

    let config = Config::guess().await.unwrap();
    let client = PodmanRestClient::new(config).await.unwrap();

    common::pull_nginx_image(&client).await;
    common::create_nginx_container(&client, "podman_rest_client_container_export_test").await;

    let stream = client
        .containers()
        .container_export_libpod("podman_rest_client_container_export_test");

    let bytes: Vec<u8> = stream.map_ok(|b| b.to_vec()).try_concat().await.unwrap();
    let c = std::io::Cursor::new(bytes);

    assert!(tar::Archive::new(c).entries().unwrap().count() > 0)
}

#[tokio::test]
async fn it_attaches_to_containers() {
    common::test_init().await;

    let config = Config::guess().await.unwrap();
    let client = PodmanRestClient::new(config).await.unwrap();

    common::pull_busybox_image(&client).await;
    let name = "podman_rest_client_attach_test";

    client
        .container_create_libpod(models::SpecGenerator {
            name: Some(name.into()),
            image: Some("docker.io/library/busybox:latest".into()),
            command: Some(vec![
                "sh".to_string(),
                "-c".to_string(),
                "while true; do echo 'Hello, World'; sleep 1; done".to_string(),
            ]),
            ..models::SpecGenerator::default()
        })
        .await
        .expect("Failed to create container");

    client.container_start_libpod(name, None).await.unwrap();

    let params = params::ContainerAttachLibpod {
        detach_keys: None,
        stderr: Some(true),
        stdout: Some(true),
        stream: Some(true),
        ..Default::default()
    };

    let conn = client
        .container_attach_libpod(name, Some(params))
        .await
        .unwrap();

    let mut reader = AttachFrameStream::new(conn);

    assert_eq!(
        reader.next().await.unwrap().unwrap(),
        AttachFrame::Stdout(Bytes::from_static(b"Hello, World\n")),
    );
    assert_eq!(
        reader.next().await.unwrap().unwrap(),
        AttachFrame::Stdout(Bytes::from_static(b"Hello, World\n")),
    );

    common::delete_container(&client, name).await;
}

#[tokio::test]
async fn it_execs_containers() {
    common::test_init().await;

    let config = Config::guess().await.unwrap();
    let client = PodmanRestClient::new(config).await.unwrap();

    common::pull_nginx_image(&client).await;
    let name = "podman_rest_client_exec_test";

    common::create_nginx_container(&client, name).await;

    client.container_start_libpod(name, None).await.unwrap();

    let exec = client
        .container_exec_libpod(
            name,
            models::ContainerExecLibpodBody {
                attach_stdout: Some(true),
                attach_stderr: Some(true),
                cmd: Some(vec!["echo".to_string(), "\"hello world\"".to_string()]),
                ..Default::default()
            },
        )
        .await
        .unwrap();

    let conn = client
        .exec_start_libpod(&exec.id, models::ExecStartLibpodBody::default())
        .await
        .unwrap();

    let mut reader = AttachFrameStream::new(conn);

    assert_eq!(
        reader.next().await.unwrap().unwrap(),
        AttachFrame::Stdout(Bytes::from_static(b"\"hello world\"\n")),
    );
    assert!(reader.next().await.is_none());

    common::delete_container(&client, name).await;
}

#[tokio::test]
async fn it_execs_a_shell_with_a_tty() {
    common::test_init().await;

    let config = Config::guess().await.unwrap();
    let client = PodmanRestClient::new(config).await.unwrap();

    common::pull_nginx_image(&client).await;
    let name = "podman_rest_client_exec_write_test";
    common::create_nginx_container(&client, name).await;

    client.container_start_libpod(name, None).await.unwrap();

    let exec = client
        .container_exec_libpod(
            name,
            models::ContainerExecLibpodBody {
                tty: Some(true),
                attach_stdin: Some(true),
                attach_stdout: Some(true),
                attach_stderr: Some(true),
                cmd: Some(vec!["sh".to_string()]),
                ..Default::default()
            },
        )
        .await
        .unwrap();

    let mut conn = client
        .exec_start_libpod(&exec.id, models::ExecStartLibpodBody::default())
        .await
        .unwrap();

    // Expect the shell prompt #
    let mut prompt_buf = [0u8; 2];
    conn.read_exact(&mut prompt_buf).await.unwrap();
    assert_eq!(&prompt_buf, b"# ");

    // Expect the stdin to be echoed back
    conn.write_all(b"echo 'tty fun'\n").await.unwrap();
    conn.flush().await.unwrap();
    let mut stdin_buf = [0u8; 16];
    conn.read_exact(&mut stdin_buf).await.unwrap();
    assert_eq!(&stdin_buf, b"echo 'tty fun'\r\n");

    // Expect the output of the command
    let mut output_buf = [0u8; 9];
    conn.read_exact(&mut output_buf).await.unwrap();
    assert_eq!(&output_buf, b"tty fun\r\n");

    common::delete_container(&client, name).await;
}

#[tokio::test]
async fn it_execs_containers_compat() {
    common::test_init().await;

    let config = Config::guess().await.unwrap();
    let client = PodmanRestClient::new(config).await.unwrap();

    common::pull_nginx_image(&client).await;
    let name = "podman_rest_client_exec_compat_test";

    common::create_nginx_container(&client, name).await;

    client.container_start_libpod(name, None).await.unwrap();

    let exec = client
        .container_exec(
            name,
            models::ContainerExecBody {
                attach_stdout: Some(true),
                attach_stderr: Some(true),
                cmd: Some(vec!["echo".to_string(), "\"hello world\"".to_string()]),
                ..Default::default()
            },
        )
        .await
        .unwrap();

    let conn = client
        .exec_start(&exec.id, models::ExecStartBody::default())
        .await
        .unwrap();

    let mut reader = AttachFrameStream::new(conn);

    assert_eq!(
        reader.next().await.unwrap().unwrap(),
        AttachFrame::Stdout(Bytes::from_static(b"\"hello world\"\n")),
    );
    assert!(reader.next().await.is_none());

    common::delete_container(&client, name).await;
}

#[tokio::test]
async fn it_can_do_a_system_ping() {
    common::test_init().await;

    let config = Config::guess().await.unwrap();
    let client = PodmanRestClient::new(config).await.unwrap();
    let string = client.system_ping().await.unwrap();
    assert_eq!("Ok", string);
}

#[tokio::test]
async fn it_can_get_system_info() {
    common::test_init().await;

    let config = Config::guess().await.unwrap();
    let client = PodmanRestClient::new(config).await.unwrap();
    let info = client.system_info_libpod().await;
    assert!(info.is_ok());
}

#[tokio::test]
async fn it_can_search_for_images_via_compat_endpoint() {
    common::test_init().await;

    let config = Config::guess().await.unwrap();
    let client = PodmanRestClient::new(config).await.unwrap();
    let res = client
        .image_search(Some(params::ImageSearch {
            term: Some("debian"),
            ..Default::default()
        }))
        .await
        .unwrap();
    assert!(res.len() > 1);
}

#[tokio::test]
async fn it_can_search_for_images() {
    common::test_init().await;

    let config = Config::guess().await.unwrap();
    let client = PodmanRestClient::new(config).await.unwrap();
    let res = client
        .image_search_libpod(Some(params::ImageSearchLibpod {
            term: Some("debian"),
            ..Default::default()
        }))
        .await
        .unwrap();
    assert!(res.len() > 1);
}

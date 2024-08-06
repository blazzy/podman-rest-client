use podman_rest_client::models::SpecGenerator;
use podman_rest_client::models::VolumeCreateOptions;
use podman_rest_client::params;
use podman_rest_client::{Config, PodmanRestClient};
use tokio::sync::OnceCell;

static TEST_INIT: OnceCell<()> = OnceCell::const_new();

pub async fn test_init() {
    TEST_INIT
        .get_or_init(|| async {
            let config = Config::guess().await.unwrap();
            let client = PodmanRestClient::new(config).await.unwrap();

            let containers = client
                .containers()
                .container_list_libpod(Some(params::ContainerListLibpod {
                    all: Some(true),
                    filters: Some(r#"{"name": ["podman_rest_client"]}"#),
                    ..Default::default()
                }))
                .await
                .unwrap();

            for container in containers {
                delete_container(&client, &container.id.unwrap()).await;
            }

            let volumes = client
                .volumes()
                .volume_list_libpod(Some(params::VolumeListLibpod {
                    filters: Some(r#"{"name": ["podman_rest_client"]}"#),
                }))
                .await
                .unwrap();

            for volume in volumes {
                delete_volume(&client, &volume.name.unwrap()).await;
            }

            let pods = client
                .pods()
                .pod_list_libpod(Some(params::PodListLibpod {
                    filters: Some(r#"{"name": ["podman_rest_client"]}"#),
                }))
                .await
                .unwrap();

            for pod in pods {
                delete_pod(&client, &pod.name.unwrap()).await;
            }
        })
        .await;
}

pub async fn pull_nginx_image(client: &PodmanRestClient) {
    client
        .images()
        .image_pull_libpod(Some(params::ImagePullLibpod {
            reference: Some("docker.io/library/nginx:latest"),
            quiet: Some(true),
            ..Default::default()
        }))
        .await
        .expect("Failed to pull image");
}

pub async fn create_nginx_container(client: &PodmanRestClient, container_name: &str) {
    let create = SpecGenerator {
        name: Some(container_name.into()),
        image: Some("docker.io/library/nginx:latest".into()),
        ..podman_rest_client::models::SpecGenerator::default()
    };

    client
        .containers()
        .container_create_libpod(create)
        .await
        .expect("Failed to create contaienr");
}

pub async fn delete_container(client: &PodmanRestClient, container_name: &str) {
    client
        .containers()
        .container_delete_libpod(container_name, None)
        .await
        .expect("Failed to clean up container");
}

pub async fn delete_pod(client: &PodmanRestClient, pod_name: &str) {
    client
        .pods()
        .pod_delete_libpod(pod_name, None)
        .await
        .expect("Failed to clean up pod");
}

pub async fn create_volume(client: &PodmanRestClient, volume_name: &str) {
    let create = VolumeCreateOptions {
        name: Some(volume_name.into()),
        ..VolumeCreateOptions::default()
    };

    client
        .volumes()
        .volume_create_libpod(create)
        .await
        .expect("Failed to create a volume");
}

pub async fn delete_volume(client: &PodmanRestClient, volume_name: &str) {
    client
        .volumes()
        .volume_delete_libpod(volume_name, None)
        .await
        .expect("Failed to clean up volume");
}

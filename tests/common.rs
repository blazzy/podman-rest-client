use podman_rest_client::models::SpecGenerator;
use podman_rest_client::models::VolumeCreateOptions;
use podman_rest_client::PodmanRestClient;

pub async fn pull_nginx_image(client: &PodmanRestClient) {
    client
        .images_api()
        .image_pull_libpod(
            Some("docker.io/library/nginx:latest"),
            Some(true),
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
        )
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
        .containers_api()
        .container_create_libpod(create)
        .await
        .expect("Failed to create contaienr");
}

pub async fn delete_container(client: &PodmanRestClient, container_name: &str) {
    client
        .containers_api()
        .container_delete_libpod(container_name, None, None, None, None, None)
        .await
        .expect("Failed to clean up container");
}

pub async fn delete_pod(client: &PodmanRestClient, pod_name: &str) {
    client
        .pods_api()
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
        .volumes_api()
        .volume_create_libpod(Some(create))
        .await
        .expect("Failed to create a volume");
}

pub async fn delete_volume(client: &PodmanRestClient, volume_name: &str) {
    client
        .volumes_api()
        .volume_delete_libpod(volume_name, None)
        .await
        .expect("Failed to clean up volume");
}

use std::process::Stdio;
use std::str::FromStr;

use serde::Deserialize;
use tokio::process::Command;

mod error;
mod ssh;
mod podman_rest_client;


pub use podman_rest_client::PodmanRestClient;
pub use podman_rest_client::Config;
pub use error::Error;

#[derive(Deserialize, Debug)]
struct PodmanConnection {
    #[serde(rename = "Name")]
    pub name: String,
    #[serde(rename = "URI")]
    pub uri: String,
    #[serde(rename = "Identity")]
    pub identity: String,
    #[serde(rename = "Default")]
    pub default: bool,
}

pub async fn get_default_system_config()  {
    let cmd = Command::new("podman")
        .args(["system", "connection", "list", "--format", "json"])
        .stdout(Stdio::piped())
        .spawn()
        .unwrap();
    let output = cmd.wait_with_output().await.unwrap();
    let string = &String::from_utf8_lossy(&output.stdout);
    println!("{}", string);

    let json: Vec<PodmanConnection> = serde_json::from_str(string).unwrap();
    let uri = hyper::Uri::from_str(&json[0].uri).unwrap();
    let authority = uri.authority().unwrap();

    println!("{}", uri.path());
    println!("{:?}", uri.host());
    println!("{:?}", uri.scheme());
    println!("{:?}", uri.port());
    println!("{:?}", authority.as_ref());
    println!("{:?}", uri);
    println!("{:?}", json);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn it_works() {
        get_default_system_config().await;

        assert_eq!(4, 4);
    }
}


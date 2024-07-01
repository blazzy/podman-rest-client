use std::process::Stdio;

use serde::Deserialize;
use tokio::process::Command;

#[derive(thiserror::Error, Debug)]
pub enum CliError {
    #[error("JSON error: {0}")]
    Json(#[from] serde_json::Error),
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
}

#[derive(Deserialize, Debug)]
pub struct PodmanConnection {
    #[serde(rename = "Name")]
    pub name: String,
    #[serde(rename = "URI")]
    pub uri: String,
    #[serde(rename = "Identity")]
    pub identity: Option<String>,
    #[serde(rename = "Default")]
    pub default: bool,
    #[serde(rename = "IsMachine")]
    pub is_machine: bool,
    #[serde(rename = "ReadWrite")]
    pub read_write: bool,
}

pub async fn get_system_connections() -> Result<Vec<PodmanConnection>, CliError> {
    let cmd = Command::new("podman")
        .args(["system", "connection", "list", "--format", "json"])
        .stdout(Stdio::piped())
        .spawn()?;
    let output = cmd.wait_with_output().await?;
    let string = &String::from_utf8_lossy(&output.stdout);

    Ok(serde_json::from_str(string)?)
}

pub async fn get_default_system_connection() -> Result<Option<PodmanConnection>, CliError> {
    let connections = get_system_connections().await?;
    Ok(connections
        .into_iter()
        .find(|connection| connection.default))
}

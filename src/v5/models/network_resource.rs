use serde::{Deserialize, Serialize};
#[derive(Default, Debug, Serialize, Deserialize)]
/// NetworkResource is the body of the "get network" http response message
pub struct NetworkResource {
    #[serde(rename = "Attachable")]
    pub attachable: Option<bool>,
    #[serde(rename = "ConfigFrom")]
    pub config_from: Option<super::super::models::ConfigReference>,
    #[serde(rename = "ConfigOnly")]
    pub config_only: Option<bool>,
    #[serde(rename = "Containers")]
    pub containers:
        Option<std::collections::HashMap<String, super::super::models::EndpointResource>>,
    #[serde(rename = "Created")]
    pub created: Option<String>,
    #[serde(rename = "Driver")]
    pub driver: Option<String>,
    #[serde(rename = "EnableIPv6")]
    pub enable_i_pv6: Option<bool>,
    #[serde(rename = "IPAM")]
    pub ipam: Option<super::super::models::Ipam>,
    #[serde(rename = "Id")]
    pub id: Option<String>,
    #[serde(rename = "Ingress")]
    pub ingress: Option<bool>,
    #[serde(rename = "Internal")]
    pub internal: Option<bool>,
    #[serde(rename = "Labels")]
    pub labels: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "Name")]
    pub name: Option<String>,
    #[serde(rename = "Options")]
    pub options: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "Peers")]
    pub peers: Option<Vec<super::super::models::PeerInfo>>,
    #[serde(rename = "Scope")]
    pub scope: Option<String>,
    #[serde(rename = "Services")]
    pub services: Option<std::collections::HashMap<String, super::super::models::ServiceInfo>>,
}

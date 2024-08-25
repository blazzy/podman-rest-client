use serde::{Deserialize, Serialize};
#[derive(Default, Debug, Serialize, Deserialize)]
/// NetworkCreate is the expected body of the "create network" http request message
pub struct NetworkCreate {
    #[serde(rename = "Attachable")]
    pub attachable: Option<bool>,
    /// Deprecated: CheckDuplicate is deprecated since API v1.44, but it defaults to true when sent by the client
    /// package to older daemons.
    #[serde(rename = "CheckDuplicate")]
    pub check_duplicate: Option<bool>,
    #[serde(rename = "ConfigFrom")]
    pub config_from: Option<crate::v5::models::ConfigReference>,
    #[serde(rename = "ConfigOnly")]
    pub config_only: Option<bool>,
    #[serde(rename = "Driver")]
    pub driver: Option<String>,
    #[serde(rename = "EnableIPv6")]
    pub enable_i_pv6: Option<bool>,
    #[serde(rename = "IPAM")]
    pub ipam: Option<crate::v5::models::Ipam>,
    #[serde(rename = "Ingress")]
    pub ingress: Option<bool>,
    #[serde(rename = "Internal")]
    pub internal: Option<bool>,
    #[serde(rename = "Labels")]
    pub labels: Option<std::collections::HashMap<String, Option<String>>>,
    #[serde(rename = "Options")]
    pub options: Option<std::collections::HashMap<String, Option<String>>>,
    #[serde(rename = "Scope")]
    pub scope: Option<String>,
}

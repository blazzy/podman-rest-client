use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Serialize, Deserialize)]
/// NetOptions reflect the shared network options between
/// pods and containers
pub struct NetOptions {
    pub dns_option: Option<Vec<String>>,

    pub dns_search: Option<Vec<String>>,

    pub dns_server: Option<Vec<String>>,

    pub hostadd: Option<Vec<String>>,

    pub netns: Option<super::super::models::Namespace>,

    pub network_alias: Option<Vec<String>>,

    /// NetworkOptions are additional options for each network
    pub network_options: Option<std::collections::HashMap<String, Vec<String>>>,

    pub networks:
        Option<std::collections::HashMap<String, super::super::models::PerNetworkOptions>>,

    pub no_manage_hosts: Option<bool>,

    pub no_manage_resolv_conf: Option<bool>,

    pub portmappings: Option<Vec<super::super::models::PortMapping>>,
}

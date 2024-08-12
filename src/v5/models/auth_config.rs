use serde::{Deserialize, Serialize};
#[derive(Default, Debug, Serialize, Deserialize)]
/// AuthConfig contains authorization information for connecting to a Registry
pub struct AuthConfig {
    pub auth: Option<String>,
    /// Email is an optional value associated with the username.
    /// This field is deprecated and will be removed in a later
    /// version of docker.
    pub email: Option<String>,
    /// IdentityToken is used to authenticate the user and get
    /// an access token for the registry.
    pub identitytoken: Option<String>,
    pub password: Option<String>,
    /// RegistryToken is a bearer token to be sent to a registry
    pub registrytoken: Option<String>,
    pub serveraddress: Option<String>,
    pub username: Option<String>,
}

use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Serialize, Deserialize)]
/// AuthenticateOKBody authenticate o k body
pub struct AuthenticateOkBody {
    /// An opaque token used to authenticate a user after a successful login
    #[serde(rename = "IdentityToken")]
    pub identity_token: String,

    /// The status of the authentication
    #[serde(rename = "Status")]
    pub status: String,
}

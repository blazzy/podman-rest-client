use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Serialize, Deserialize)]
/// RemoteSocket describes information about the API socket
pub struct RemoteSocket {
    pub exists: Option<bool>,

    pub path: Option<String>,
}

use std::fmt;
use std::fmt::Debug;

use hyper;
use hyper::http;
use hyper_util::client::legacy::connect::Connect;
use serde_json;

#[derive(Debug)]
pub enum Error {
    Api(ApiError),
    Header(http::header::InvalidHeaderValue),
    Http(http::Error),
    Hyper(hyper::Error),
    HyperClient(hyper_util::client::legacy::Error),
    Serde(serde_json::Error),
    UriError(http::uri::InvalidUri),
}

pub struct ApiError {
    pub code: hyper::StatusCode,
    pub body: hyper::body::Incoming,
}

impl Debug for ApiError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("ApiError")
         .field("code", &self.code)
         .field("body", &"hyper::body::Incoming")
         .finish()
    }
}

impl From<(hyper::StatusCode, hyper::body::Incoming)> for Error {
    fn from(e: (hyper::StatusCode, hyper::body::Incoming)) -> Self {
        Error::Api(ApiError {
            code: e.0,
            body: e.1,
        })
    }
}

impl From<http::Error> for Error {
    fn from(e: http::Error) -> Self {
        Error::Http(e)
    }
}

impl From<hyper_util::client::legacy::Error> for Error {
    fn from(e: hyper_util::client::legacy::Error) -> Self {
        Error::HyperClient(e)
    }
}

impl From<hyper::Error> for Error {
    fn from(e: hyper::Error) -> Self {
        Error::Hyper(e)
    }
}

impl From<serde_json::Error> for Error {
    fn from(e: serde_json::Error) -> Self {
        Error::Serde(e)
    }
}

mod request;

mod containers_api;
pub use self::containers_api::{ ContainersApi, ContainersApiClient };
mod containers_compat_api;
pub use self::containers_compat_api::{ ContainersCompatApi, ContainersCompatApiClient };
mod exec_api;
pub use self::exec_api::{ ExecApi, ExecApiClient };
mod exec_compat_api;
pub use self::exec_compat_api::{ ExecCompatApi, ExecCompatApiClient };
mod images_api;
pub use self::images_api::{ ImagesApi, ImagesApiClient };
mod images_compat_api;
pub use self::images_compat_api::{ ImagesCompatApi, ImagesCompatApiClient };
mod manifests_api;
pub use self::manifests_api::{ ManifestsApi, ManifestsApiClient };
mod networks_api;
pub use self::networks_api::{ NetworksApi, NetworksApiClient };
mod networks_compat_api;
pub use self::networks_compat_api::{ NetworksCompatApi, NetworksCompatApiClient };
mod pods_api;
pub use self::pods_api::{ PodsApi, PodsApiClient };
mod secrets_api;
pub use self::secrets_api::{ SecretsApi, SecretsApiClient };
mod secrets_compat_api;
pub use self::secrets_compat_api::{ SecretsCompatApi, SecretsCompatApiClient };
mod system_api;
pub use self::system_api::{ SystemApi, SystemApiClient };
mod system_compat_api;
pub use self::system_compat_api::{ SystemCompatApi, SystemCompatApiClient };
mod volumes_api;
pub use self::volumes_api::{ VolumesApi, VolumesApiClient };
mod volumes_compat_api;
pub use self::volumes_compat_api::{ VolumesCompatApi, VolumesCompatApiClient };

pub mod configuration;
pub mod client;

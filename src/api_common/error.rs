use std::fmt;

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("Api Error: {code} {body}")]
    Api {
        code: hyper::StatusCode,
        body: ApiErrorBody,
    },

    #[error("URL Parse Error: {0}")]
    UrlParse(#[from] url::ParseError),

    #[error("Invalid URI: {0}")]
    InvalidUri(#[from] hyper::http::uri::InvalidUri),

    #[error("HTTP Error: {0}")]
    HttpError(#[from] hyper::http::Error),

    #[error("Hyper Client Error: {0}")]
    HyperClient(#[from] hyper_util::client::legacy::Error),

    #[error("Hyper Error: {0}")]
    Hyper(#[from] hyper::Error),

    #[error("JSON Error: {0}")]
    SerdeJson(#[from] serde_json::Error),

    #[error("JSON Error: {0}")]
    SerdeJsonPath(#[from] serde_path_to_error::Error<serde_json::Error>),
}

#[derive(Debug)]
pub enum ApiErrorBody {
    Json(serde_json::Value),
    Plain(String),
}

impl From<hyper::body::Bytes> for ApiErrorBody {
    fn from(bytes: hyper::body::Bytes) -> ApiErrorBody {
        serde_json::from_slice(&bytes)
            .map(ApiErrorBody::Json)
            .unwrap_or_else(|_| ApiErrorBody::Plain(String::from_utf8_lossy(&bytes).to_string()))
    }
}

impl fmt::Display for ApiErrorBody {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ApiErrorBody::Json(json) => write!(f, "{}", json),
            ApiErrorBody::Plain(str) => write!(f, "{}", str),
        }
    }
}

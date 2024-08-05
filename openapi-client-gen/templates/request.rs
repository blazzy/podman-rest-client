use http_body_util::BodyExt;
use serde_path_to_error::deserialize;

use crate::config::ClientConfig;
use crate::error::Error;

pub async fn execute_request_bytes(
    config: &dyn ClientConfig,
    request: http::request::Request<String>,
) -> Result<hyper::body::Bytes, Error> {
    let response = config.request(request).await?;
    let status = response.status();
    let bytes = response.into_body().collect().await?.to_bytes();
    if status.is_success() {
        Ok(bytes)
    } else {
        Err(Error::Api {
            code: status,
            body: bytes.into(),
        })
    }
}

pub async fn execute_request_unit(
    config: &dyn ClientConfig,
    request: http::request::Request<String>,
) -> Result<(), Error> {
    execute_request_bytes(config, request).await?;
    Ok(())
}

pub async fn execute_request_json<U>(
    config: &dyn ClientConfig,
    request: http::request::Request<String>,
) -> Result<U, Error>
where
    for<'a> U: serde::Deserialize<'a>,
{
    let bytes = execute_request_bytes(config, request).await?;
    let deserializer = &mut serde_json::Deserializer::from_slice(&bytes);

    Ok(deserialize(deserializer)?)
}

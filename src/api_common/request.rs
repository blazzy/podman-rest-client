use futures::stream;
use futures::stream::StreamExt;
use futures::Stream;
use http_body_util::BodyExt;
use serde_path_to_error::deserialize;
use std::pin::Pin;

use super::config::ClientConfig;
use super::error::Error;

pub async fn execute_request_unit(
    config: &dyn ClientConfig,
    request: Result<http::request::Request<String>, Error>,
) -> Result<(), Error> {
    execute_request_bytes(config, request).await?;
    Ok(())
}

pub async fn execute_request_json<U>(
    config: &dyn ClientConfig,
    request: Result<http::request::Request<String>, Error>,
) -> Result<U, Error>
where
    for<'a> U: serde::Deserialize<'a>,
{
    let bytes = execute_request_bytes(config, request).await?;
    let deserializer = &mut serde_json::Deserializer::from_slice(&bytes);

    Ok(deserialize(deserializer)?)
}

pub async fn execute_request_bytes(
    config: &dyn ClientConfig,
    request: Result<http::request::Request<String>, Error>,
) -> Result<hyper::body::Bytes, Error> {
    let response = config.request(request?).await?;
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

pub fn execute_request_stream<'a>(
    config: &'a dyn ClientConfig,
    request: Result<http::request::Request<String>, Error>,
) -> Pin<Box<dyn Stream<Item = Result<bytes::Bytes, Error>> + 'a + Send>> {
    let result = async move {
        let response = config.request(request?).await?;
        let status = response.status();
        let body = response.into_body();

        if status.is_success() {
            Ok(Box::pin(body.into_data_stream().map(
                |result| match result {
                    Ok(bytes) => Ok(bytes),
                    Err(err) => Err(err.into()),
                },
            )))
        } else {
            let bytes = body.collect().await?.to_bytes();
            Err(Error::Api {
                code: status,
                body: bytes.into(),
            })
        }
    };
    Box::pin(stream::once(result).flat_map(|result| match result {
        Ok(stream) => stream,
        Err(err) => Box::pin(stream::once(async { Err(err) }))
            as Pin<Box<dyn Stream<Item = Result<_, Error>> + Send>>,
    }))
}

use futures::stream;
use futures::stream::StreamExt;
use futures::Stream;
use http::request::Builder;
use http_body_util::BodyExt;
use hyper::header;
use hyper::upgrade::Upgraded;
use hyper_util::rt::TokioIo;
use serde_path_to_error::deserialize;
use std::pin::Pin;

use super::config::ClientConfig;
use super::error::Error;

pub async fn execute_request_unit<F>(config: &dyn ClientConfig, request: F) -> Result<(), Error>
where
    F: Fn(Builder) -> Result<http::request::Request<String>, Error>,
{
    execute_request_bytes(config, request).await?;
    Ok(())
}

pub async fn execute_request_json<U, F>(config: &dyn ClientConfig, request: F) -> Result<U, Error>
where
    for<'a> U: serde::Deserialize<'a>,
    F: Fn(Builder) -> Result<http::request::Request<String>, Error>,
{
    let bytes = execute_request_bytes(config, request).await?;
    let deserializer = &mut serde_json::Deserializer::from_slice(&bytes);

    Ok(deserialize(deserializer)?)
}

pub async fn execute_request_bytes<F>(
    config: &dyn ClientConfig,
    request: F,
) -> Result<hyper::body::Bytes, Error>
where
    F: Fn(Builder) -> Result<http::request::Request<String>, Error>,
{
    let response = config.request(request(config.req_builder())?).await?;
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

pub fn execute_request_stream<'a, F>(
    config: &'a dyn ClientConfig,
    request: F,
) -> Pin<Box<dyn Stream<Item = Result<bytes::Bytes, Error>> + 'a + Send>>
where
    F: Fn(Builder) -> Result<http::request::Request<String>, Error> + 'a + Send,
{
    let result = async move {
        let builder = config.req_builder();
        let response = config.request(request(builder)?).await?;
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

pub async fn execute_request_upgrade<F>(
    config: &dyn ClientConfig,
    request: F,
) -> Result<TokioIo<Upgraded>, Error>
where
    F: Fn(Builder) -> Result<http::request::Request<String>, Error>,
{
    let builder = config
        .req_builder()
        .header(header::CONNECTION, "Upgrade")
        .header(header::UPGRADE, "websocket");
    let response = config.request(request(builder)?).await?;
    let status = response.status();

    if response.status() == hyper::StatusCode::SWITCHING_PROTOCOLS {
        match hyper::upgrade::on(response).await {
            Ok(upgraded) => Ok(hyper_util::rt::TokioIo::new(upgraded)),
            Err(e) => Err(e.into()),
        }
    } else {
        let bytes = response.into_body().collect().await?.to_bytes();
        Err(Error::Api {
            code: status,
            body: bytes.into(),
        })
    }
}

use futures::stream;
use futures::stream::StreamExt;
use futures::Stream;
use http::request::Builder;
use http_body_util::BodyExt;
use hyper::body::Bytes;
use hyper::header;
use hyper::upgrade::Upgraded;
use hyper_util::rt::TokioIo;
use serde_path_to_error::deserialize;
use std::io;
use std::pin::Pin;
use std::task::{Context, Poll};
use tokio::io::AsyncRead;
use tokio::io::ReadBuf;

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

#[derive(Debug, PartialEq, Eq)]
pub enum AttachFrame {
    Stdin(Bytes),
    Stdout(Bytes),
    Stderr(Bytes),
}

pub struct AttachFrameStream<R> {
    reader: R,
    buf: Vec<u8>,
    pos: usize,
    state: AttachFrameStreamState,
}

enum AttachFrameStreamState {
    ReadingHeader,
    ReadingData { frame_type: u8, size: usize },
}

impl<R: AsyncRead + Unpin> AttachFrameStream<R> {
    pub fn new(reader: R) -> Self {
        AttachFrameStream {
            reader,
            buf: vec![0; 8],
            pos: 0,
            state: AttachFrameStreamState::ReadingHeader,
        }
    }
}

impl<R: AsyncRead + Unpin> Stream for AttachFrameStream<R> {
    type Item = io::Result<AttachFrame>;

    fn poll_next(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        loop {
            let AttachFrameStream {
                reader, buf, pos, ..
            } = &mut *self;

            let reader = Pin::new(reader);
            let mut read_buf = ReadBuf::new(&mut buf[*pos..]);
            let poll = reader.poll_read(cx, &mut read_buf);
            let n = read_buf.filled().len();
            self.pos += n;

            match poll {
                Poll::Ready(Err(e)) => return Poll::Ready(Some(Err(e))),
                Poll::Pending => return Poll::Pending,
                Poll::Ready(Ok(())) => {
                    if n == 0 {
                        return Poll::Ready(None);
                    }

                    match self.state {
                        AttachFrameStreamState::ReadingHeader => {
                            if self.pos == self.buf.len() {
                                let frame_type = self.buf[0];
                                let frame_size = u32::from_be_bytes([
                                    self.buf[4],
                                    self.buf[5],
                                    self.buf[6],
                                    self.buf[7],
                                ]) as usize;
                                self.buf = vec![0; frame_size];
                                self.pos = 0;

                                self.state = AttachFrameStreamState::ReadingData {
                                    frame_type,
                                    size: frame_size,
                                };
                            }
                        }
                        AttachFrameStreamState::ReadingData { frame_type, size } => {
                            if self.pos == size {
                                let frame = match frame_type {
                                    0x00 => AttachFrame::Stdin(self.buf.clone().into()),
                                    0x01 => AttachFrame::Stdout(self.buf.clone().into()),
                                    0x02 => AttachFrame::Stderr(self.buf.clone().into()),
                                    _ => {
                                        return Poll::Ready(Some(Err(io::Error::new(
                                            io::ErrorKind::InvalidData,
                                            "Unknown frame type",
                                        ))))
                                    }
                                };
                                self.buf = vec![0; 8];
                                self.pos = 0;

                                self.state = AttachFrameStreamState::ReadingHeader;
                                return Poll::Ready(Some(Ok(frame)));
                            }
                        }
                    }
                }
            }
        }
    }
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

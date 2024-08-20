use std::future::Future;
use std::io;
use std::pin::Pin;
use std::task::{Context, Poll};

use hyper::rt::ReadBufCursor;
use hyper::Uri;
use hyper_util::client::legacy::connect::{Connected, Connection};
use tokio::io::AsyncRead;
use tokio::io::AsyncWrite;
use tokio::net::UnixStream;
use tower_service::Service;

use crate::api_common::Connector;
use crate::error::ClientError;

#[derive(Clone)]
pub(crate) struct UnixConnector {
    path: String,
}

pub struct UnixStreamWrapper(pub UnixStream);

impl UnixConnector {
    pub fn new(path: &str) -> UnixConnector {
        UnixConnector {
            path: path.to_string(),
        }
    }
}

impl Connector for UnixConnector {}

impl Service<Uri> for UnixConnector {
    type Response = UnixStreamWrapper;
    type Error = ClientError;
    type Future = Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>> + Send>>;

    fn poll_ready(&mut self, _cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        Poll::Ready(Ok(()))
    }

    fn call(&mut self, _req: Uri) -> Self::Future {
        let path = self.path.clone();
        let future = async move {
            let stream = UnixStreamWrapper(UnixStream::connect(path).await?);

            Ok(stream)
        };
        Box::pin(future)
    }
}

impl Connection for UnixStreamWrapper {
    fn connected(&self) -> Connected {
        Connected::new()
    }
}

impl hyper::rt::Read for UnixStreamWrapper {
    fn poll_read(
        self: Pin<&mut Self>,
        cx: &mut Context<'_>,
        mut buf: ReadBufCursor<'_>,
    ) -> Poll<std::io::Result<()>> {
        let mut temp_buf = unsafe { tokio::io::ReadBuf::uninit(buf.as_mut()) };

        match Pin::new(&mut self.get_mut().0).poll_read(cx, &mut temp_buf) {
            Poll::Ready(Ok(())) => {
                let n = temp_buf.filled().len();
                unsafe {
                    buf.advance(n);
                }
                Poll::Ready(Ok(()))
            }
            other => other,
        }
    }
}

impl hyper::rt::Write for UnixStreamWrapper {
    fn poll_write(
        self: Pin<&mut Self>,
        cx: &mut Context<'_>,
        buf: &[u8],
    ) -> Poll<io::Result<usize>> {
        Pin::new(&mut self.get_mut().0).poll_write(cx, buf)
    }

    fn poll_flush(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<io::Result<()>> {
        Pin::new(&mut self.get_mut().0).poll_flush(cx)
    }

    fn poll_shutdown(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<io::Result<()>> {
        Pin::new(&mut self.get_mut().0).poll_shutdown(cx)
    }
}

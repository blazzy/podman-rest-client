use std::future::Future;
use std::io;
use std::pin::Pin;
use std::sync::Arc;
use std::task::{Context, Poll};

use hyper::rt::ReadBufCursor;
use hyper::Uri;
use hyper_util::client::legacy::connect::{Connected, Connection};
use russh::client as ssh_client;
use russh::client::Config;
use russh::client::Msg;
use russh::ChannelStream;
use russh_keys::key;
use tokio::io::{AsyncRead, AsyncWrite};
use tower_service::Service;

use crate::api_common::Connector;
use crate::error::ClientError;

#[derive(Clone)]
pub(crate) struct SshConnector {
    session: Arc<ssh_client::Handle<Client>>,
    socket_path: String,
}

impl Connector for SshConnector {}

impl SshConnector {
    pub async fn new(
        user: &str,
        key: &str,
        address: &str,
        socket_path: &str,
    ) -> Result<SshConnector, ClientError> {
        let config = Arc::new(Config::default());
        let sh = Client {};
        let mut session = ssh_client::connect(config, address, sh).await?;
        let key_pair = russh_keys::load_secret_key(key, None)?;
        let auth_res = session
            .authenticate_publickey(user, Arc::new(key_pair))
            .await?;
        if !auth_res {
            return Err(ClientError::AuthenticationFailed);
        }
        Ok(SshConnector {
            session: Arc::new(session),
            socket_path: socket_path.to_string(),
        })
    }
}

impl Service<Uri> for SshConnector {
    type Response = WrapChannelStream;
    type Error = ClientError;
    type Future = Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>> + Send>>;

    fn poll_ready(&mut self, _cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        Poll::Ready(Ok(()))
    }

    fn call(&mut self, _req: Uri) -> Self::Future {
        let session = self.session.clone();
        let socket_path = self.socket_path.clone();

        let future = async move {
            let socket_channel = session.channel_open_direct_streamlocal(socket_path).await?;

            Ok(WrapChannelStream(socket_channel.into_stream()))
        };
        Box::pin(future)
    }
}

pub struct WrapChannelStream(pub ChannelStream<Msg>);

impl Connection for WrapChannelStream {
    fn connected(&self) -> Connected {
        Connected::new()
    }
}

impl hyper::rt::Read for WrapChannelStream {
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

struct Client {}

#[async_trait::async_trait]
impl ssh_client::Handler for Client {
    type Error = russh::Error;

    async fn check_server_key(
        &mut self,
        _server_public_key: &key::PublicKey,
    ) -> Result<bool, russh::Error> {
        Ok(true)
    }
}

impl hyper::rt::Write for WrapChannelStream {
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

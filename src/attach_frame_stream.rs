use std::io;
use std::pin::Pin;
use std::task::{Context, Poll};

use futures::Stream;
use hyper::body::Bytes;
use tokio::io::AsyncRead;
use tokio::io::ReadBuf;

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

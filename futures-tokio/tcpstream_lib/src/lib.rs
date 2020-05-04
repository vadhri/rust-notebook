use tokio::prelude::*;
use tokio::net::TcpStream;
use tokio::stream::Stream;

use pin_project::pin_project;

use std::pin::Pin;
use std::task::{Context, Poll};

use futures::executor;

#[pin_project]
pub struct StreamReader {
    source: TcpStream,
}

impl StreamReader {
    pub fn new(r: TcpStream) -> Self {
        StreamReader { source: r }
    }
}

impl Stream for StreamReader {
    type Item = Vec<u8>;

    fn poll_next(self: Pin<&mut Self>, _cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        let mut this = self.project();
        let unpinned_source: &mut TcpStream = &mut this.source;
        let mut bytes_read: Result<usize, io::Error> = Ok(0);
        let mut buffer = [0; 1024];

        let ft = async {
            bytes_read = unpinned_source.read(&mut buffer).await;
        };

        executor::block_on(ft);

        match bytes_read {
            Ok(n) => {
                if n > 0 {
                    return Poll::Ready(Some(buffer[0..n].to_vec()));
                } else if n == 0 {
                    return Poll::Ready(None);
                } else {
                    return Poll::Pending;
                }
            }
            Err(_reason) => {
                return Poll::Ready(None);
            }
        }
    }
}

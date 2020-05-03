use tokio::net::TcpListener;
use tokio::prelude::*;
use tokio::stream::StreamExt;

use tokio::net::TcpStream;
use tokio::stream::Stream;

use pin_project::pin_project;

use std::pin::Pin;
use std::task::{Context, Poll};

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

use futures::executor;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let ft = async {
        let mut listener = TcpListener::bind("127.0.0.1:6142").await.unwrap();
        loop {
            match listener.accept().await {
                Ok((_socket, addr)) => {
                    println!("{:?}", addr);
                    
                    let async_block = async {
                        let mut reader_stream = StreamReader::new(_socket);
                        let mut value = vec![0; 1];
                        let mut total_bytes_transfered = 0;

                        while value.len() > 0 {
                            let buff_value = reader_stream.next().await;

                            if buff_value != None {
                                value = buff_value.unwrap();
                                total_bytes_transfered += value.len();
                            } else {
                                value.clear();
                            }
                        }
                        println!("{:?} bytes transferred.", total_bytes_transfered);
                    };
                    tokio::spawn(async_block);
                }
                Err(e) => println!("couldn't get client: {:?}", e),
            }
        }
    };

    ft.await;

    Ok(())
}

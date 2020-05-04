use tokio::net::TcpListener;
use tokio::prelude::*;
use tokio::stream::StreamExt;

use tokio::net::TcpStream;
use tokio::stream::Stream;

use std::pin::Pin;
use std::task::{Context, Poll};

pub struct StreamReader {
    source: TcpStream,
}

impl StreamReader {
    pub fn new(r: TcpStream) -> Self {
        StreamReader { source: r }
    }

    pub async fn respond(self: &mut Self, s: &[u8]) {
        self.source.write(&s);
        self.source.flush();
    }
}

impl Stream for StreamReader {
    type Item = Vec<u8>;

    fn poll_next(mut self: Pin<&mut Self>, _cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        let mut buffer = [0; 2048];

        match Pin::new(&mut self.source).poll_read(_cx, &mut buffer) {
            Poll::Ready(Ok(0)) => Poll::Ready(None),
            Poll::Ready(Ok(m)) => Poll::Ready(Some(buffer[0..m].to_vec())),
            Poll::Ready(Err(_reason)) => Poll::Ready(None),
            Poll::Pending => Poll::Pending
        }
    }
}

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
                                reader_stream.respond(format!("{:?} bytes transferred.", value.len()).as_bytes()).await;
                                println!("Received >> {:?}", String::from_utf8(value.clone()).unwrap());
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

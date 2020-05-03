use tokio::stream::StreamExt;
use tokio::net::TcpStream;
use tokio::prelude::*;

use tokio::net::TcpListener;

use futures::channel::mpsc;
use futures::executor;
use futures::executor::*;

use std::str;

#[tokio::main]
async fn main() {
    let addr = "127.0.0.1:8080";
    let mut listener = TcpListener::bind(addr).await.unwrap();

    let server = async move {
        let mut incoming = listener.incoming();
        while let Some(socket_res) = incoming.next().await {
            match socket_res {
                Ok(mut socket) => {
                    let mut buf:Vec<u8> = Vec::new();
                    let mut total_buffer_read = 0u32;

                    loop {
                        let mut buff = [0u8; 64];
                        let n = socket.read(&mut buff).await.unwrap();
                        total_buffer_read += n as u32;

                        if n == 0 {
                            break
                        } else {
                            println!("Partial data ... {:?} bytes", n);
                            buf.append(&mut buff.to_vec());
                        }
                    }

                    println!("Read all data ... {:?} bytes", String::from_utf8(buf[0..(total_buffer_read as usize)].to_vec()).unwrap().len());
                }
                Err(err) => {
                    println!("accept error = {:?}", err);
                }
            }
        }
    };

    println!("Server running on localhost:6142");

    server.await;
}

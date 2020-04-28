use tokio::stream::StreamExt;
use tokio::net::TcpStream;
use tokio::prelude::*;

use tokio::net::TcpListener;

use futures::channel::mpsc;
use futures::executor;
use futures::executor::*;

pub enum op {
    ADD,
    SUB,
    MUL,
    DIV
}

pub struct task {
    operation: op,
    op1: u32,
    op2: u32
}

#[tokio::main]
async fn main() {
    let addr = "127.0.0.1:6142";
    let mut listener = TcpListener::bind(&addr).await?;
    println!("Listening on: {}", addr);

    loop {
        let (mut socket, _) = listener.accept().await?;

        tokio::spawn(async move {
            let mut buf = [0; 1024];

            // In a loop, read data from the socket and write the data back.
            loop {
                let n = socket
                    .read(&mut buf)
                    .await
                    .expect("failed to read data from socket");

                if n == 0 {
                    return;
                } else {
                    println!("Received -> {:?}", buf);
                }

                socket
                    .write_all(&buf[0..n])
                    .await
                    .expect("failed to write data to socket");
            }
        });
    }
}

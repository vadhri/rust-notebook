use tokio::{net::TcpStream};
use tokio_util::codec::{Framed, BytesCodec};

use futures::sink::SinkExt;

use std::io;
use tokio::stream::StreamExt;

use std::io::Read;
use bytes::Bytes;

use tokio::task;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let socket = TcpStream::connect("127.0.0.1:6142").await.unwrap();
    let mut socket_wrapped = Framed::new(socket, BytesCodec::new());

    loop {
        let mut buffer = task::spawn_blocking(|| -> Result<String, io::Error> {
            let mut input_string = String::new();
            io::stdin().read_to_string(&mut input_string)?;
            Ok(input_string)
        }).await?.unwrap();

        buffer.truncate(buffer.len() - 1);

        match buffer.len() {
            n if n > 0 => {
                let _res = socket_wrapped.send(Bytes::from(buffer)).await;
                let _res = socket_wrapped.flush().await;
            },
            _rest => {
                break
            }
        }

        let buffer = socket_wrapped.next().await;
        let rcvd = buffer.unwrap().unwrap();
        println!(
            "Socket buffer -> {:?}",
            String::from_utf8(rcvd.to_vec()).unwrap()
        );
    }

    Ok(())
}

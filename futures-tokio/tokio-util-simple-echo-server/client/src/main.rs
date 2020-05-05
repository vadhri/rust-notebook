use tokio::{net::TcpStream};
use tokio_util::codec::{Decoder, Encoder, Framed, BytesCodec};

use bytes::{BufMut, BytesMut};
use futures::sink::SinkExt;

use std::io;

use std::io::Read;
use bytes::Bytes;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let socket = TcpStream::connect("127.0.0.1:6142").await.unwrap();
    let mut socket_wrapped = Framed::new(socket, BytesCodec::new());

    loop {
        let mut buffer = String::new();
        let _res = io::stdin().read_to_string(&mut buffer)?;

        match _res {
            n if n > 0 => {
                let _res = socket_wrapped.send(Bytes::from(buffer)).await;
                let _res = socket_wrapped.flush().await;
            },
            _rest => {
                break
            }
        }
    }

    Ok(())
}

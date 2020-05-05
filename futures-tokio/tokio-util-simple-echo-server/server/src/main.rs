use tokio::stream::StreamExt;
use tokio_util::codec::{Decoder, Encoder, Framed, BytesCodec};

use bytes::{BufMut, BytesMut};
use futures::sink::SinkExt;

use tokio::net::TcpListener;
use tokio::prelude::*;
use bytes::Bytes;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let mut listener = TcpListener::bind("127.0.0.1:6142").await?;
    println!("Listening on port 6142 ..");

    loop {
        let (socket, _) = listener.accept().await?;
        let mut socket_wrapped = Framed::new(socket, BytesCodec::new());

        loop {
            let buffer = socket_wrapped.next().await;
            let rcvd = buffer.unwrap().unwrap();
            println!(
                "Socket buffer -> {:?}",
                String::from_utf8(rcvd.to_vec()).unwrap()
            );
            if rcvd.len() > 0 {
                match socket_wrapped
                    .send(rcvd.freeze())
                    .await
                {
                    Ok(()) => {
                        let _ignore = socket_wrapped.flush().await;
                    }
                    _rest => {
                        break;
                    }
                }
            } else {
                println!("zero bytes {:?}", rcvd.len());
                break;
            }
        }
    }
}

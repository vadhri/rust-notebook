use tokio::stream::StreamExt;
use tokio_util::codec::{Decoder, Encoder};

use bytes::{BufMut, BytesMut};
use futures::sink::SinkExt;

use tokio::net::TcpListener;
use tokio::prelude::*;

pub struct ByteCodec;

impl Decoder for ByteCodec {
    type Item = Vec<u8>;
    type Error = io::Error;

    fn decode(&mut self, buf: &mut BytesMut) -> Result<Option<Vec<u8>>, io::Error> {
        let len = buf.len();
        Ok(Some(buf.split_to(len).to_vec()))
    }
}

impl Encoder<&[u8]> for ByteCodec {
    type Error = io::Error;

    fn encode(&mut self, data: &[u8], buf: &mut BytesMut) -> Result<(), io::Error> {
        buf.reserve(data.len());
        buf.put_slice(data);
        Ok(())
    }
}

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let mut listener = TcpListener::bind("127.0.0.1:6142").await?;
    println!("Listening on port 6142 ..");

    loop {
        let (socket, _) = listener.accept().await?;
        let mut socket_wrapped = ByteCodec.framed(socket);

        loop {
            let buffer = socket_wrapped.next().await;
            let rcvd = buffer.unwrap().unwrap();
            println!(
                "Socket buffer -> {:?}",
                String::from_utf8(rcvd.clone()).unwrap()
            );
            if rcvd.len() > 0 {
                match socket_wrapped
                    .send(String::from_utf8(rcvd).unwrap().as_bytes())
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

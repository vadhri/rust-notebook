use tokio::io::AsyncReadExt;
use tokio::io::AsyncWriteExt;
use tokio::{net::TcpStream, net::UdpSocket, stream::StreamExt};
use tokio_util::codec::{Decoder, Encoder, Framed, FramedRead, FramedWrite};
use tokio_util::udp::UdpFramed;

use bytes::{BufMut, BytesMut};
use futures::future::try_join;
use futures::future::FutureExt;
use futures::sink::SinkExt;

use futures::Stream;

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
    let mut socket = TcpStream::connect("127.0.0.1:6142").await?;
    println!("Listening on port 6142 ..");
    let mut socket_wrapped = ByteCodec.framed(socket);

    loop {
        let mut buffer = String::new();
        io::stdin().read_to_string(&mut buffer);

        let length = socket_wrapped.send(buffer.as_bytes()).await;
        println!("buffer length = {:?}", length);
    }
}

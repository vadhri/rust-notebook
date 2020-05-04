use tokio::net::TcpStream;
use tokio::prelude::*;

use std::fs::File;
use std::io::prelude::*;

#[tokio::main]
async fn main() {
    let mut stream = TcpStream::connect("127.0.0.1:6142").await.unwrap();
    println!("created stream");

    let filename = "input.txt";
    let mut f = File::open(&filename).expect("no file found");
    let mut total_buffer_written = 0;

    loop {
        let mut buffer = vec![0; stream.send_buffer_size().unwrap()];
        let mut n = f.read(&mut buffer).expect("buffer overflow") as u32;

        if n == 0 {
            break;
        }

        let mut bytes_written = 0;

        while n > 0 {
            let result = stream
                .write(&buffer[bytes_written..(n as usize + bytes_written)])
                .await;
            let bytes_sent = result.unwrap();

            n -= bytes_sent as u32;
            bytes_written += bytes_sent;

            total_buffer_written += bytes_sent;

            println!("wrote to stream; success={:?} left to send, {:?} stream.send_buffer_size().unwrap() = {:?}",n, bytes_written, stream.send_buffer_size().unwrap());
        }
    }
}

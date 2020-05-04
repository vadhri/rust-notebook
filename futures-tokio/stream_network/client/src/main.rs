use tokio::net::TcpStream;
use tokio::prelude::*;

use std::fs::File;
use std::io::prelude::*;

#[tokio::main]
async fn main() {
    println!("created stream");

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

    let mut data = [0u8; 1024];
    let data_read = stream.read(&mut data).await.unwrap();

    println!(">> Server Response(sent = {:?}) => {:?} ", total_buffer_written, String::from_utf8(data[0..data_read].to_vec()).unwrap());

    println!("chat -> CTRL+D to send");

    loop {
        let mut buffer = String::new();
        let bytes_read = io::stdin().read_to_string(&mut buffer).await;

        buffer.truncate(bytes_read.unwrap() - 1);

        if buffer == "end".to_string() {
            break;
        }

        stream.write(buffer.as_bytes()).await;
        stream.flush();

        let mut data = [0u8; 1024];
        let data_read = stream.read(&mut data).await.unwrap();

        println!(">> Server Response(sent = {:?}) => {:?} ", total_buffer_written, String::from_utf8(data[0..data_read].to_vec()).unwrap());
    }
}

use tokio::net::TcpStream;
use tokio::prelude::*;

use std::fs::File;
use std::fs;
use std::io::prelude::*;

#[tokio::main]
async fn main() {
    let mut stream = TcpStream::connect("127.0.0.1:6142").await.unwrap();
    println!("created stream");

    let filename = "input.txt";
    let mut f = File::open(&filename).expect("no file found");
    let metadata = fs::metadata(&filename).expect("unable to read metadata");
    let mut total_buffer_read = 0;

    loop {
        let mut buffer = vec![0; 64 as usize];

        let n = f.read(&mut buffer).expect("buffer overflow");

        if n == 0 {
            break
        }

        let result = stream.write(& buffer[0..n]).await;

        total_buffer_read += n;

        println!("wrote to stream; success={:?}", result);
    }
}

use std::env;
use std::io::{Read, Write};
use std::str::from_utf8;
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt, BufReader},
    net::TcpStream,
};
use util;

const HOST: &str = "localhost";
const PORT: &str = "2202";

#[tokio::main]
async fn main() {
    let addr = format!("{}:{}", HOST, PORT);

    match TcpStream::connect(&addr).await {
        Ok(mut stream) => {
            println!("Connected in ({})\n", addr);

            let mut buffer = [0u8; 11];

            stream.read(&mut buffer).await.unwrap();

            if from_utf8(&buffer).unwrap() == "CHECK_ALIVE" {
                stream.write_all("ALIVE".as_bytes()).await.unwrap();
            }

            util::download_file("", stream).await.unwrap();
        }
        Err(e) => {
            println!("Bad connection: {}", e)
        }
    }
}

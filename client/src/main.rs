use std::env;
use std::io::{Read, Write};
use std::str::from_utf8;
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt, BufReader},
    net::TcpStream,
};
use util;
use whoami;

const HOST: &str = "localhost";
const PORT: &str = "2202";

#[tokio::main]
async fn main() {
    let addr = format!("{}:{}", HOST, PORT);

    match TcpStream::connect(&addr).await {
        Ok(mut stream) => {
            println!("Connected in ({})\n", addr);

            let (os, username, device_name) =
                (whoami::distro(), whoami::username(), whoami::devicename());

            stream.write_all(os.as_bytes()).await.unwrap();
            stream.write_all(username.as_bytes()).await.unwrap();
            stream.write_all(device_name.as_bytes()).await.unwrap();
        }
        Err(e) => {
            println!("Bad connection: {}", e)
        }
    }
}

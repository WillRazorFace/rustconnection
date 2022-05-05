use std::process;
use tokio::net::TcpStream;
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

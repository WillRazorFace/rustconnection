use std::io;
use std::io::{Read, Write};
use std::net::TcpStream;
use std::str::from_utf8;

const HOST: &str = "localhost";
const PORT: &str = "2202";

fn receive_data(mut stream: &TcpStream) -> io::Result<String> {
    let mut buffer = [0u8; 1024];

    let bytes_read = stream.read(&mut buffer).unwrap();
    stream.flush().unwrap();

    Ok(String::from(from_utf8(&buffer[..bytes_read]).unwrap()))
}

fn send_data(mut stream: &TcpStream, data: String) {
    stream.write_all(data.as_bytes()).unwrap();
    stream.flush().unwrap();
}

fn main() {
    let addr = format!("{}:{}", HOST, PORT);

    match TcpStream::connect(&addr) {
        Ok(stream) => {
            println!("Connected in ({})\n", addr);
            loop {
                let response: String = receive_data(&stream).unwrap();

                if response == "CHECK_ALIVE" {
                    send_data(&stream, String::from(""));
                }

                loop {}
            }
        }
        Err(e) => {
            println!("Bad connection: {}", e);
        }
    }
}

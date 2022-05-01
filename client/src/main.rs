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
            println!("Connected in ({})! Enjoy.\n", addr);
            loop {
                print!("(You) > ");
                io::stdout().flush().unwrap();

                let mut data = String::new();

                io::stdin().read_line(&mut data).unwrap();
                io::stdout().flush().unwrap();

                send_data(&stream, data);

                let response: String = receive_data(&stream).unwrap();

                println!("({}) > {}", addr, response);
                io::stdout().flush().unwrap();
            }
        }
        Err(e) => {
            println!("Bad connection: {}", e);
        }
    }
}
